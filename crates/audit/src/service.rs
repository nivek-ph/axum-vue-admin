use std::collections::BTreeMap;

use serde::Serialize;
use sqlx::{FromRow, PgConnection, PgPool, Row};
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{AuditError, AuditEvent, AuditEventView, AuditQuery};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditDailyStat {
    pub date: String,
    pub logins: i64,
    pub unique_ips: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditHourlyStat {
    pub hour: i16,
    pub logins: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditNamedCount {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AuditStats {
    pub days: i64,
    pub login_count: i64,
    pub unique_ips: i64,
    pub event_count: i64,
    pub daily: Vec<AuditDailyStat>,
    pub by_hour: Vec<AuditHourlyStat>,
    pub top_actions: Vec<AuditNamedCount>,
    pub top_ips: Vec<AuditNamedCount>,
}

#[derive(Clone)]
pub struct AuditService {
    pool: PgPool,
}

impl AuditService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn record(&self, event: AuditEvent) -> Result<(), AuditError> {
        let mut connection = self.pool.acquire().await?;
        Self::record_in(&mut connection, event).await
    }

    pub async fn record_best_effort(&self, event: AuditEvent) {
        let action = event.action.to_string();
        if let Err(error) = self.record(event).await {
            tracing::error!(action, error = ?error, "audit event write failed");
        }
    }

    pub async fn record_in(conn: &mut PgConnection, event: AuditEvent) -> Result<(), AuditError> {
        let action = event.action.to_string();
        let result = event.result.to_string();
        let reason_code = event.reason_code.map(|code| code.to_string());
        let changes = serde_json::to_value(event.changes)?;
        sqlx::query(
            r#"
            insert into sys_audit_events (
                req_id, actor_id, actor_label, action, resource_type, resource_id,
                result, reason_code, source_ip, user_agent, changes
            ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(event.req_id)
        .bind(event.actor.id)
        .bind(event.actor.label)
        .bind(action)
        .bind(event.resource.resource_type())
        .bind(event.resource.resource_id())
        .bind(result)
        .bind(reason_code)
        .bind(event.source.ip)
        .bind(event.source.user_agent)
        .bind(changes)
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn list(
        &self,
        query: AuditQuery,
    ) -> Result<(Vec<AuditEventView>, i64, i64, i64), AuditError> {
        let started_at = parse_time(query.started_at.as_deref())?;
        let ended_at = parse_time(query.ended_at.as_deref())?;
        let page = query.page.max(1);
        let page_size = query.page_size.max(1);
        let offset = (page - 1) * page_size;
        let total = sqlx::query_scalar::<_, i64>(
            r#"
            select count(*) from sys_audit_events
            where ($1::text is null or req_id ilike '%' || $1 || '%')
              and ($2::text is null or actor_label ilike '%' || $2 || '%' or actor_id::text = $2)
              and ($3::text is null or action = $3)
              and ($4::text is null or resource_type = $4)
              and ($5::text is null or resource_id = $5)
              and ($6::text is null or result = $6)
              and ($7::timestamptz is null or created_at >= $7)
              and ($8::timestamptz is null or created_at <= $8)
            "#,
        )
        .bind(query.req_id.as_deref())
        .bind(query.actor.as_deref())
        .bind(query.action.as_deref())
        .bind(query.resource_type.as_deref())
        .bind(query.resource_id.as_deref())
        .bind(query.result.as_deref())
        .bind(started_at)
        .bind(ended_at)
        .fetch_one(&self.pool)
        .await?;

        let events = sqlx::query_as::<_, AuditEventView>(
            r#"
            select
                id, req_id, actor_id, actor_label, action, resource_type, resource_id, result,
                reason_code, source_ip, user_agent, changes,
                to_char(created_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at
            from sys_audit_events
            where ($1::text is null or req_id ilike '%' || $1 || '%')
              and ($2::text is null or actor_label ilike '%' || $2 || '%' or actor_id::text = $2)
              and ($3::text is null or action = $3)
              and ($4::text is null or resource_type = $4)
              and ($5::text is null or resource_id = $5)
              and ($6::text is null or result = $6)
              and ($7::timestamptz is null or created_at >= $7)
              and ($8::timestamptz is null or created_at <= $8)
            order by id desc
            limit $9 offset $10
            "#,
        )
        .bind(query.req_id.as_deref())
        .bind(query.actor.as_deref())
        .bind(query.action.as_deref())
        .bind(query.resource_type.as_deref())
        .bind(query.resource_id.as_deref())
        .bind(query.result.as_deref())
        .bind(started_at)
        .bind(ended_at)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((events, total, page, page_size))
    }

    pub async fn find(&self, id: i64) -> Result<Option<AuditEventView>, AuditError> {
        Ok(sqlx::query_as::<_, AuditEventView>(
            r#"
            select
                id, req_id, actor_id, actor_label, action, resource_type, resource_id, result,
                reason_code, source_ip, user_agent, changes,
                to_char(created_at at time zone 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at
            from sys_audit_events
            where id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?)
    }

    pub async fn stats(&self, days: i64) -> Result<AuditStats, AuditError> {
        let days = days.clamp(1, 90);
        let started_at = OffsetDateTime::now_utc() - Duration::days(days);

        #[derive(FromRow)]
        struct SummaryRow {
            login_count: i64,
            unique_ips: i64,
            event_count: i64,
        }

        let summary = sqlx::query_as::<_, SummaryRow>(
            r#"
            select
                count(*) filter (where action = 'auth.login') as login_count,
                count(distinct nullif(source_ip, '')) as unique_ips,
                count(*) as event_count
            from sys_audit_events
            where created_at >= $1
            "#,
        )
        .bind(started_at)
        .fetch_one(&self.pool)
        .await?;

        let daily_login_rows = sqlx::query(
            r#"
            select to_char((created_at at time zone 'UTC')::date, 'YYYY-MM-DD') as day,
                   count(*)::bigint as logins
            from sys_audit_events
            where created_at >= $1
              and action = 'auth.login'
            group by 1
            order by 1
            "#,
        )
        .bind(started_at)
        .fetch_all(&self.pool)
        .await?;

        let daily_ip_rows = sqlx::query(
            r#"
            select to_char((created_at at time zone 'UTC')::date, 'YYYY-MM-DD') as day,
                   count(distinct nullif(source_ip, ''))::bigint as unique_ips
            from sys_audit_events
            where created_at >= $1
            group by 1
            order by 1
            "#,
        )
        .bind(started_at)
        .fetch_all(&self.pool)
        .await?;

        let hourly_rows = sqlx::query(
            r#"
            select extract(hour from created_at at time zone 'UTC')::smallint as hour,
                   count(*)::bigint as logins
            from sys_audit_events
            where created_at >= $1
              and action = 'auth.login'
            group by 1
            order by 1
            "#,
        )
        .bind(started_at)
        .fetch_all(&self.pool)
        .await?;

        let top_action_rows = sqlx::query(
            r#"
            select action as name, count(*)::bigint as count
            from sys_audit_events
            where created_at >= $1
            group by action
            order by count desc, action asc
            limit 10
            "#,
        )
        .bind(started_at)
        .fetch_all(&self.pool)
        .await?;

        let top_ip_rows = sqlx::query(
            r#"
            select source_ip as name, count(*)::bigint as count
            from sys_audit_events
            where created_at >= $1
              and source_ip <> ''
            group by source_ip
            order by count desc, source_ip asc
            limit 10
            "#,
        )
        .bind(started_at)
        .fetch_all(&self.pool)
        .await?;

        let mut login_by_day = BTreeMap::<String, i64>::new();
        for row in &daily_login_rows {
            login_by_day.insert(row.try_get("day")?, row.try_get("logins")?);
        }
        let mut ips_by_day = BTreeMap::<String, i64>::new();
        for row in &daily_ip_rows {
            ips_by_day.insert(row.try_get("day")?, row.try_get("unique_ips")?);
        }

        let today = OffsetDateTime::now_utc().date();
        let mut daily = Vec::with_capacity(days as usize);
        for offset in (0..days).rev() {
            let date = today - Duration::days(offset);
            let key = date.to_string();
            daily.push(AuditDailyStat {
                date: key.clone(),
                logins: login_by_day.get(&key).copied().unwrap_or(0),
                unique_ips: ips_by_day.get(&key).copied().unwrap_or(0),
            });
        }

        let mut logins_by_hour = [0_i64; 24];
        for row in &hourly_rows {
            let hour: i16 = row.try_get("hour")?;
            let logins: i64 = row.try_get("logins")?;
            if (0..24).contains(&hour) {
                logins_by_hour[hour as usize] = logins;
            }
        }
        let by_hour = (0_i16..24)
            .map(|hour| AuditHourlyStat {
                hour,
                logins: logins_by_hour[hour as usize],
            })
            .collect();

        let top_actions = named_counts(top_action_rows)?;
        let top_ips = named_counts(top_ip_rows)?;

        Ok(AuditStats {
            days,
            login_count: summary.login_count,
            unique_ips: summary.unique_ips,
            event_count: summary.event_count,
            daily,
            by_hour,
            top_actions,
            top_ips,
        })
    }
}

fn named_counts(rows: Vec<sqlx::postgres::PgRow>) -> Result<Vec<AuditNamedCount>, AuditError> {
    rows.into_iter()
        .map(|row| {
            Ok(AuditNamedCount {
                name: row.try_get("name")?,
                count: row.try_get("count")?,
            })
        })
        .collect()
}

fn parse_time(value: Option<&str>) -> Result<Option<OffsetDateTime>, AuditError> {
    value
        .map(|value| OffsetDateTime::parse(value, &Rfc3339).map_err(AuditError::InvalidTimeRange))
        .transpose()
}
