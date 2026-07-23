use audit::{
    AuditAction, AuditActor, AuditEvent, AuditQuery, AuditResource, AuditResult, AuditService,
    AuditSource,
};
use sqlx::PgPool;

fn login_event(result: AuditResult, ip: &str) -> AuditEvent {
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
            ip: ip.to_string(),
            user_agent: "audit-test".to_string(),
        },
        changes: Vec::new(),
    }
}

fn assign_roles_event(ip: &str) -> AuditEvent {
    AuditEvent {
        actor: AuditActor {
            id: Some(7),
            label: "admin".to_string(),
        },
        action: AuditAction::AssignUserRoles,
        resource: AuditResource::User(9),
        result: AuditResult::Succeeded,
        reason_code: None,
        source: AuditSource {
            ip: ip.to_string(),
            user_agent: "audit-test".to_string(),
        },
        changes: Vec::new(),
    }
}

#[sqlx::test(migrations = "../../migrations")]
async fn fresh_schema_records_and_filters_structured_audit_events(pool: PgPool) {
    let service = AuditService::new(pool.clone());
    service
        .record(login_event(AuditResult::Succeeded, "127.0.0.1"))
        .await
        .expect("audit event should be recorded");
    sqlx::query(
        "update sys_audit_events set created_at = '2026-07-16T15:30:00+08:00'::timestamptz",
    )
    .execute(&pool)
    .await
    .expect("audit event timestamp should be fixed for UTC assertions");
    sqlx::query("set time zone 'Asia/Shanghai'")
        .execute(&pool)
        .await
        .expect("audit queries should run in a non-UTC database session");

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
    assert_eq!(events[0].created_at, "2026-07-16T07:30:00Z");

    let item = service
        .find(events[0].id)
        .await
        .expect("audit detail query should succeed")
        .expect("audit event should exist");
    assert_eq!(item.id, events[0].id);
    assert_eq!(item.created_at, "2026-07-16T07:30:00Z");
}

#[sqlx::test(migrations = "../../migrations")]
async fn stats_aggregates_logins_ips_hours_and_top_actions(pool: PgPool) {
    let service = AuditService::new(pool.clone());
    service
        .record(login_event(AuditResult::Succeeded, "10.0.0.1"))
        .await
        .unwrap();
    service
        .record(login_event(AuditResult::Failed, "10.0.0.2"))
        .await
        .unwrap();
    service
        .record(assign_roles_event("10.0.0.1"))
        .await
        .unwrap();

    sqlx::query(
        r#"
        update sys_audit_events
        set created_at = case id % 3
            when 1 then now() - interval '2 days' + interval '10 hours'
            when 2 then now() - interval '1 day' + interval '15 hours'
            else now() - interval '3 hours'
        end
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let stats = service.stats(14).await.expect("stats should succeed");
    assert_eq!(stats.days, 14);
    assert_eq!(stats.login_count, 2);
    assert_eq!(stats.event_count, 3);
    assert_eq!(stats.unique_ips, 2);
    assert_eq!(stats.daily.len(), 14);
    assert_eq!(stats.by_hour.len(), 24);
    assert!(stats.by_hour.iter().map(|row| row.logins).sum::<i64>() >= 2);
    assert_eq!(stats.top_actions.len(), 2);
    assert_eq!(stats.top_actions[0].name, "auth.login");
    assert_eq!(stats.top_actions[0].count, 2);
    assert_eq!(stats.top_ips[0].name, "10.0.0.1");
    assert_eq!(stats.top_ips[0].count, 2);
}
