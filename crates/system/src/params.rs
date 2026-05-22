use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SysParam {
    #[serde(rename = "ID", default)]
    pub id: i64,
    pub name: String,
    pub key: String,
    pub value: String,
    pub desc: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdRequest {
    #[serde(rename = "ID", alias = "id")]
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdsRequest {
    #[serde(rename = "IDs", alias = "ids")]
    pub ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ParamListQuery {
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
    pub name: Option<String>,
    pub key: Option<String>,
}

pub async fn list(
    pool: &sqlx::PgPool,
    query: ParamListQuery,
) -> Result<(Vec<SysParam>, i64, i64, i64), sqlx::Error> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).max(1);
    let offset = (page - 1) * page_size;
    let total: i64 = sqlx::query_scalar(
        r#"
        select count(*) from sys_params
        where ($1::text is null or name ilike '%' || $1 || '%')
          and ($2::text is null or "key" ilike '%' || $2 || '%')
        "#,
    )
    .bind(query.name.as_deref())
    .bind(query.key.as_deref())
    .fetch_one(pool)
    .await?;

    let list = sqlx::query_as::<_, SysParam>(
        r#"
        select id, name, "key", value, "desc"
        from sys_params
        where ($1::text is null or name ilike '%' || $1 || '%')
          and ($2::text is null or "key" ilike '%' || $2 || '%')
        order by id desc
        limit $3 offset $4
        "#,
    )
    .bind(query.name.as_deref())
    .bind(query.key.as_deref())
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((list, total, page, page_size))
}

pub async fn create(pool: &sqlx::PgPool, payload: SysParam) -> Result<(), sqlx::Error> {
    sqlx::query("insert into sys_params (name, \"key\", value, \"desc\") values ($1, $2, $3, $4)")
        .bind(payload.name)
        .bind(payload.key)
        .bind(payload.value)
        .bind(payload.desc)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(pool: &sqlx::PgPool, payload: SysParam) -> Result<(), sqlx::Error> {
    sqlx::query(
        "update sys_params set name = $1, \"key\" = $2, value = $3, \"desc\" = $4 where id = $5",
    )
    .bind(payload.name)
    .bind(payload.key)
    .bind(payload.value)
    .bind(payload.desc)
    .bind(payload.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find(pool: &sqlx::PgPool, id: i64) -> Result<Option<SysParam>, sqlx::Error> {
    sqlx::query_as::<_, SysParam>(
        "select id, name, \"key\", value, \"desc\" from sys_params where id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_params where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_many(pool: &sqlx::PgPool, ids: Vec<i64>) -> Result<(), sqlx::Error> {
    sqlx::query("delete from sys_params where id = any($1)")
        .bind(ids)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_by_key(pool: &sqlx::PgPool, key: &str) -> Result<Option<SysParam>, sqlx::Error> {
    sqlx::query_as::<_, SysParam>(
        "select id, name, \"key\", value, \"desc\" from sys_params where \"key\" = $1",
    )
    .bind(key)
    .fetch_optional(pool)
    .await
}
