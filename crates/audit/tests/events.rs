use audit::{
    AuditAction, AuditActor, AuditEvent, AuditQuery, AuditResource, AuditResult, AuditService,
    AuditSource,
};
use sqlx::PgPool;

fn login_event(result: AuditResult) -> AuditEvent {
    AuditEvent {
        actor: AuditActor {
            id: Some(7),
            label: "admin".to_string(),
        },
        action: AuditAction::Login,
        resource: AuditResource::Account("admin".to_string()),
        result,
        reason_code: None,
        source: AuditSource {
            ip: "127.0.0.1".to_string(),
            user_agent: "audit-test".to_string(),
        },
        changes: Vec::new(),
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn unified_audit_store_records_and_filters_structured_events(pool: PgPool) {
    let service = AuditService::new(pool.clone());
    service
        .record(login_event(AuditResult::Succeeded))
        .await
        .expect("audit event should be recorded");

    let (events, total, page, page_size) = service
        .list(AuditQuery {
            page: 1,
            page_size: 10,
            actor: Some("admin".to_string()),
            action: Some("auth.login".to_string()),
            resource_type: Some("account".to_string()),
            resource_id: Some("admin".to_string()),
            result: Some("succeeded".to_string()),
            started_at: Some("2000-01-01T00:00:00Z".to_string()),
            ended_at: Some("2100-01-01T00:00:00Z".to_string()),
        })
        .await
        .expect("audit events should be queryable");

    assert_eq!(total, 1);
    assert_eq!(page, 1);
    assert_eq!(page_size, 10);
    assert_eq!(events[0].actor_label, "admin");
    assert_eq!(events[0].action, "auth.login");
    assert_eq!(events[0].resource_type, "account");
    assert_eq!(events[0].resource_id.as_deref(), Some("admin"));
    assert_eq!(events[0].result, "succeeded");
    assert_eq!(events[0].source_ip, "127.0.0.1");

    let item = service
        .find(events[0].id)
        .await
        .expect("audit detail query should succeed")
        .expect("audit event should exist");
    assert_eq!(item.id, events[0].id);

    let old_tables: (Option<String>, Option<String>) = sqlx::query_as(
        "select to_regclass('sys_login_logs')::text, to_regclass('sys_operation_records')::text",
    )
    .fetch_one(&pool)
    .await
    .expect("table names should be inspectable");
    assert_eq!(old_tables, (None, None));
}

#[sqlx::test(migrations = false)]
async fn migration_preserves_legacy_login_and_operation_records(pool: PgPool) {
    for migration in [
        include_str!("../../../migrations/0001_init.sql"),
        include_str!("../../../migrations/0002_rbac.sql"),
        include_str!("../../../migrations/0003_seed.sql"),
    ] {
        sqlx::raw_sql(migration)
            .execute(&pool)
            .await
            .expect("pre-unified migration should apply");
    }

    sqlx::query(
        r#"
        insert into sys_login_logs (
            username, ip, status, error_message, agent, user_id, created_at
        ) values (
            'admin', '10.0.0.1', false, 'invalid username or password', 'legacy-login', 1,
            '2026-01-01T00:00:00Z'
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        r#"
        insert into sys_operation_records (
            ip, method, path, status, agent, error_message, body, resp, user_id, created_at
        ) values (
            '10.0.0.2', 'PUT', '/api/users/7/authorities', 403, 'legacy-operation',
            'forbidden', '{"password":"must-not-migrate"}', 'denied', 1,
            '2026-01-02T00:00:00Z'
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::raw_sql(include_str!(
        "../../../migrations/0004_unified_audit_events.sql"
    ))
    .execute(&pool)
    .await
    .expect("unified audit migration should apply");

    let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
        r#"
        select action, resource_id, result, coalesce(reason_code, ''), source_ip, user_agent
        from sys_audit_events
        order by created_at
        "#,
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        rows,
        vec![
            (
                "auth.login".to_string(),
                "admin".to_string(),
                "denied".to_string(),
                "invalid_credentials".to_string(),
                "10.0.0.1".to_string(),
                "legacy-login".to_string(),
            ),
            (
                "legacy.http_request".to_string(),
                "/api/users/7/authorities".to_string(),
                "failed".to_string(),
                "http_status_403".to_string(),
                "10.0.0.2".to_string(),
                "legacy-operation".to_string(),
            ),
        ]
    );

    let serialized = sqlx::query_scalar::<_, String>(
        "select jsonb_agg(to_jsonb(e))::text from sys_audit_events e",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(!serialized.contains("must-not-migrate"));
    let old_tables: (Option<String>, Option<String>) = sqlx::query_as(
        "select to_regclass('sys_login_logs')::text, to_regclass('sys_operation_records')::text",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(old_tables, (None, None));
}
