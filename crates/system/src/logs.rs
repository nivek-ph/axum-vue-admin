use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone)]
pub struct CreateLoginLog {
    pub username: String,
    pub ip: String,
    pub status: bool,
    pub error_message: String,
    pub agent: String,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct CreateOperationLog {
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status: i32,
    pub agent: String,
    pub error_message: String,
    pub body: String,
    pub resp: String,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginLogSearch {
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub username: Option<String>,
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OperationLogSearch {
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdRequest {
    #[serde(rename = "ID")]
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdsRequest {
    pub ids: Vec<i64>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct LoginLogView {
    #[serde(rename = "ID")]
    pub id: i64,
    pub username: String,
    pub ip: String,
    pub status: bool,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    pub agent: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationUserView {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct OperationLogRow {
    pub id: i64,
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status: i32,
    pub agent: String,
    pub error_message: String,
    pub body: String,
    pub resp: String,
    pub created_at: String,
    pub user_name: String,
    pub nick_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationLogView {
    #[serde(rename = "ID")]
    pub id: i64,
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status: i32,
    pub agent: String,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    pub body: String,
    pub resp: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    pub user: OperationUserView,
}

pub async fn create_login_log(pool: &sqlx::PgPool, log: CreateLoginLog) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into sys_login_logs (username, ip, status, error_message, agent, user_id)
        values ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(log.username)
    .bind(log.ip)
    .bind(log.status)
    .bind(log.error_message)
    .bind(log.agent)
    .bind(log.user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn create_operation_log(
    pool: &sqlx::PgPool,
    log: CreateOperationLog,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into sys_operation_records (
            ip, method, path, status, agent, error_message, body, resp, user_id
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(log.ip)
    .bind(log.method)
    .bind(log.path)
    .bind(log.status)
    .bind(log.agent)
    .bind(log.error_message)
    .bind(log.body)
    .bind(log.resp)
    .bind(log.user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_login_log_list(
    pool: &sqlx::PgPool,
    query: LoginLogSearch,
) -> Result<(Vec<LoginLogView>, i64), sqlx::Error> {
    let page = query.page.max(1);
    let page_size = query.page_size.max(1);
    let offset = (page - 1) * page_size;
    let total: i64 = sqlx::query_scalar(
        r#"
        select count(*) from sys_login_logs
        where ($1::text is null or username ilike '%' || $1 || '%')
          and ($2::bool is null or status = $2)
        "#,
    )
    .bind(query.username.as_deref())
    .bind(query.status)
    .fetch_one(pool)
    .await?;

    let list = sqlx::query_as::<_, LoginLogView>(
        r#"
        select
            id,
            username,
            ip,
            status,
            error_message,
            agent,
            to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS') as created_at
        from sys_login_logs
        where ($1::text is null or username ilike '%' || $1 || '%')
          and ($2::bool is null or status = $2)
        order by id desc
        limit $3 offset $4
        "#,
    )
    .bind(query.username.as_deref())
    .bind(query.status)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((list, total))
}

pub async fn delete_login_log(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_login_logs where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_login_logs(pool: &sqlx::PgPool, ids: Vec<i64>) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_login_logs where id = any($1)")
        .bind(ids)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_operation_log_list(
    pool: &sqlx::PgPool,
    query: OperationLogSearch,
) -> Result<(Vec<OperationLogView>, i64), sqlx::Error> {
    let page = query.page.max(1);
    let page_size = query.page_size.max(1);
    let offset = (page - 1) * page_size;
    let total: i64 = sqlx::query_scalar(
        r#"
        select count(*) from sys_operation_records
        where ($1::text is null or method ilike '%' || $1 || '%')
          and ($2::text is null or path ilike '%' || $2 || '%')
          and ($3::int is null or status = $3)
        "#,
    )
    .bind(query.method.as_deref())
    .bind(query.path.as_deref())
    .bind(query.status)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, OperationLogRow>(
        r#"
        select
            r.id,
            r.ip,
            r.method,
            r.path,
            r.status,
            r.agent,
            r.error_message,
            r.body,
            r.resp,
            to_char(r.created_at, 'YYYY-MM-DD"T"HH24:MI:SS') as created_at,
            coalesce(u.username, '') as user_name,
            coalesce(u.nick_name, '') as nick_name
        from sys_operation_records r
        left join sys_users u on u.id = r.user_id
        where ($1::text is null or r.method ilike '%' || $1 || '%')
          and ($2::text is null or r.path ilike '%' || $2 || '%')
          and ($3::int is null or r.status = $3)
        order by r.id desc
        limit $4 offset $5
        "#,
    )
    .bind(query.method.as_deref())
    .bind(query.path.as_deref())
    .bind(query.status)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let list = rows
        .into_iter()
        .map(|row| OperationLogView {
            id: row.id,
            ip: row.ip,
            method: row.method,
            path: row.path,
            status: row.status,
            agent: row.agent,
            error_message: row.error_message,
            body: row.body,
            resp: row.resp,
            created_at: row.created_at,
            user: OperationUserView {
                user_name: row.user_name,
                nick_name: row.nick_name,
            },
        })
        .collect();

    Ok((list, total))
}

pub async fn delete_operation_log(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_operation_records where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_operation_logs(pool: &sqlx::PgPool, ids: Vec<i64>) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_operation_records where id = any($1)")
        .bind(ids)
        .execute(pool)
        .await?;
    Ok(())
}
