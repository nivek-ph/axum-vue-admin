use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentCategory {
    #[serde(rename = "ID", default)]
    pub id: i64,
    pub name: String,
    pub pid: i64,
    pub children: Vec<AttachmentCategory>,
}

#[derive(Debug, Clone, FromRow)]
struct AttachmentCategoryRow {
    pub id: i64,
    pub name: String,
    pub pid: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryPayload {
    #[serde(rename = "ID", default)]
    pub id: i64,
    pub name: String,
    pub pid: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteCategoryPayload {
    pub id: i64,
}

pub async fn list(pool: &sqlx::PgPool) -> Result<Vec<AttachmentCategory>, sqlx::Error> {
    let rows = sqlx::query_as::<_, AttachmentCategoryRow>(
        "select id, name, pid from attachment_categories order by id asc",
    )
    .fetch_all(pool)
    .await?;
    Ok(build_tree(&rows, 0))
}

pub async fn upsert(pool: &sqlx::PgPool, payload: CategoryPayload) -> Result<(), sqlx::Error> {
    if payload.id == 0 {
        sqlx::query("insert into attachment_categories (name, pid) values ($1, $2)")
            .bind(payload.name)
            .bind(payload.pid)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("update attachment_categories set name = $1, pid = $2 where id = $3")
            .bind(payload.name)
            .bind(payload.pid)
            .bind(payload.id)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn delete(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("delete from uploaded_files where class_id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    sqlx::query("delete from attachment_categories where id = $1 or pid = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

fn build_tree(rows: &[AttachmentCategoryRow], pid: i64) -> Vec<AttachmentCategory> {
    let mut list = rows
        .iter()
        .filter(|row| row.pid == pid)
        .map(|row| {
            let mut item = AttachmentCategory {
                id: row.id,
                name: row.name.clone(),
                pid: row.pid,
                children: Vec::new(),
            };
            item.children = build_tree(rows, row.id);
            item
        })
        .collect::<Vec<_>>();
    list.sort_by_key(|item| item.id);
    list
}
