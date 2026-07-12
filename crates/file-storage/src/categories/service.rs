use super::model::AttachmentCategoryRow;
use super::{AttachmentCategory, CategoryError, CategoryPayload};

#[derive(Clone)]
pub struct CategoryService {
    pool: sqlx::PgPool,
}

impl CategoryService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
    pub async fn list(&self) -> Result<Vec<AttachmentCategory>, CategoryError> {
        Ok(list(&self.pool).await?)
    }
    pub async fn upsert(&self, payload: CategoryPayload) -> Result<(), CategoryError> {
        Ok(upsert(&self.pool, payload).await?)
    }
    pub async fn delete(&self, id: i64) -> Result<(), CategoryError> {
        Ok(delete(&self.pool, id).await?)
    }
}

pub(crate) async fn list(pool: &sqlx::PgPool) -> Result<Vec<AttachmentCategory>, sqlx::Error> {
    let rows = sqlx::query_as::<_, AttachmentCategoryRow>(
        "select id, name, pid from attachment_categories order by id asc",
    )
    .fetch_all(pool)
    .await?;
    Ok(build_tree(&rows, 0))
}

pub(crate) async fn upsert(
    pool: &sqlx::PgPool,
    payload: CategoryPayload,
) -> Result<(), sqlx::Error> {
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

pub(crate) async fn delete(pool: &sqlx::PgPool, id: i64) -> Result<(), sqlx::Error> {
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
