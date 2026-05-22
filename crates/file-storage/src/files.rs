use std::path::PathBuf;

use admin_httpz::AppError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::errors;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct StoredFile {
    #[serde(rename = "ID")]
    pub id: i64,
    pub name: String,
    pub url: String,
    pub tag: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    #[serde(rename = "classId")]
    pub class_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileListQuery {
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub keyword: Option<String>,
    #[serde(rename = "classId")]
    pub class_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileEditPayload {
    #[serde(rename = "ID")]
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileDeletePayload {
    #[serde(rename = "ID")]
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImportUrlPayload {
    pub name: String,
    pub url: String,
    #[serde(rename = "classId")]
    pub class_id: Option<i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl From<FileError> for AppError {
    fn from(error: FileError) -> Self {
        match error {
            FileError::Database(error) => errors::FILE_DB_FAILED.into_error().with_source(error),
            FileError::Io(error) => errors::FILE_IO_FAILED.into_error().with_source(error),
        }
    }
}

pub async fn list(
    pool: &sqlx::PgPool,
    query: FileListQuery,
) -> Result<(Vec<StoredFile>, i64, i64, i64), FileError> {
    let page = query.page.max(1);
    let page_size = query.page_size.max(1);
    let offset = (page - 1) * page_size;
    let total: i64 = sqlx::query_scalar(
        r#"
        select count(*) from uploaded_files
        where ($1::text is null or name ilike '%' || $1 || '%' or url ilike '%' || $1 || '%')
          and ($2::bigint is null or class_id = $2)
        "#,
    )
    .bind(query.keyword.as_deref())
    .bind(query.class_id)
    .fetch_one(pool)
    .await?;
    let list = sqlx::query_as::<_, StoredFile>(
        r#"
        select
            id,
            name,
            url,
            tag,
            to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS') as updated_at,
            class_id
        from uploaded_files
        where ($1::text is null or name ilike '%' || $1 || '%' or url ilike '%' || $1 || '%')
          and ($2::bigint is null or class_id = $2)
        order by id desc
        limit $3 offset $4
        "#,
    )
    .bind(query.keyword.as_deref())
    .bind(query.class_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok((list, total, page, page_size))
}

pub async fn edit_name(pool: &sqlx::PgPool, payload: FileEditPayload) -> Result<(), FileError> {
    sqlx::query("update uploaded_files set name = $1, updated_at = now() where id = $2")
        .bind(payload.name)
        .bind(payload.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_file(pool: &sqlx::PgPool, id: i64) -> Result<(), FileError> {
    sqlx::query("delete from uploaded_files where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn import_url(pool: &sqlx::PgPool, payload: ImportUrlPayload) -> Result<(), FileError> {
    let tag = payload
        .url
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_string();
    sqlx::query("insert into uploaded_files (name, url, tag, class_id) values ($1, $2, $3, $4)")
        .bind(payload.name)
        .bind(payload.url)
        .bind(tag)
        .bind(payload.class_id.unwrap_or(0))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn store_uploaded_bytes(
    pool: &sqlx::PgPool,
    upload_dir: &str,
    file_name: &str,
    class_id: i64,
    bytes: &[u8],
) -> Result<StoredFile, FileError> {
    tokio::fs::create_dir_all(upload_dir).await?;
    let extension = file_name.rsplit('.').next().unwrap_or_default();
    let generated = format!("{}-{}", Uuid::new_v4(), file_name);
    let mut path = PathBuf::from(upload_dir);
    path.push(&generated);
    let mut file = tokio::fs::File::create(&path).await?;
    file.write_all(bytes).await?;
    file.flush().await?;
    let url = format!("/uploads/{generated}");

    let id: i64 = sqlx::query_scalar(
        r#"
        insert into uploaded_files (name, url, tag, class_id)
        values ($1, $2, $3, $4)
        returning id
        "#,
    )
    .bind(file_name)
    .bind(&url)
    .bind(extension)
    .bind(class_id)
    .fetch_one(pool)
    .await?;

    Ok(sqlx::query_as::<_, StoredFile>(
        r#"
        select
            id,
            name,
            url,
            tag,
            to_char(updated_at, 'YYYY-MM-DD"T"HH24:MI:SS') as updated_at,
            class_id
        from uploaded_files
        where id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?)
}
