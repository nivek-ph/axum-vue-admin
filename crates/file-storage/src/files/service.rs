use std::path::{Path, PathBuf};

use sqlx::PgPool;
use uuid::Uuid;

use super::{FileEditPayload, FileError, FileListQuery, ImportUrlPayload, StoredFile};

#[derive(Clone)]
pub struct FileService {
    pool: PgPool,
    upload_dir: String,
}

impl FileService {
    pub fn new(pool: PgPool, upload_dir: impl Into<String>) -> Self {
        Self {
            pool,
            upload_dir: upload_dir.into(),
        }
    }
    pub async fn list(
        &self,
        query: FileListQuery,
    ) -> Result<(Vec<StoredFile>, i64, i64, i64), FileError> {
        list(&self.pool, query).await
    }
    pub async fn edit_name(&self, payload: FileEditPayload) -> Result<(), FileError> {
        edit_name(&self.pool, payload).await
    }
    pub async fn import_url(&self, payload: ImportUrlPayload) -> Result<(), FileError> {
        import_url(&self.pool, payload).await
    }
    pub async fn upload(
        &self,
        name: &str,
        class_id: i64,
        bytes: &[u8],
    ) -> Result<StoredFile, FileError> {
        store_uploaded_bytes(&self.pool, &self.upload_dir, name, class_id, bytes).await
    }
    pub async fn delete(&self, id: i64) -> Result<(), FileError> {
        let Some(file) = find_file(&self.pool, id).await? else {
            return Ok(());
        };
        let staged = self.stage_local_file(&file.url).await?;
        if let Err(error) = delete_file(&self.pool, id).await {
            if let Some((original, staged)) = staged {
                tokio::fs::rename(staged, original).await?;
            }
            return Err(error);
        }
        if let Some((_, staged)) = staged {
            match tokio::fs::remove_file(staged).await {
                Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
                    return Err(FileError::Io(error));
                }
                _ => {}
            }
        }
        Ok(())
    }
    async fn stage_local_file(&self, url: &str) -> Result<Option<(PathBuf, PathBuf)>, FileError> {
        if !url.starts_with("/uploads/") {
            return Ok(None);
        }
        let Some(name) = Path::new(url).file_name() else {
            return Ok(None);
        };
        let original = Path::new(&self.upload_dir).join(name);
        let staged = original.with_extension(format!("deleting-{}", Uuid::new_v4()));
        match tokio::fs::rename(&original, &staged).await {
            Ok(()) => Ok(Some((original, staged))),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(FileError::Io(error)),
        }
    }
}
use tokio::io::AsyncWriteExt;

pub(crate) async fn list(
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

pub(crate) async fn edit_name(
    pool: &sqlx::PgPool,
    payload: FileEditPayload,
) -> Result<(), FileError> {
    sqlx::query("update uploaded_files set name = $1, updated_at = now() where id = $2")
        .bind(payload.name)
        .bind(payload.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn find_file(
    pool: &sqlx::PgPool,
    id: i64,
) -> Result<Option<StoredFile>, FileError> {
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
    .fetch_optional(pool)
    .await?)
}

pub(crate) async fn delete_file(pool: &sqlx::PgPool, id: i64) -> Result<(), FileError> {
    sqlx::query("delete from uploaded_files where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn import_url(
    pool: &sqlx::PgPool,
    payload: ImportUrlPayload,
) -> Result<(), FileError> {
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

pub(crate) async fn store_uploaded_bytes(
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

    let id_result: Result<i64, sqlx::Error> = sqlx::query_scalar(
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
    .await;
    let id = match id_result {
        Ok(id) => id,
        Err(error) => {
            match tokio::fs::remove_file(&path).await {
                Err(cleanup_error) if cleanup_error.kind() != std::io::ErrorKind::NotFound => {
                    return Err(FileError::Io(cleanup_error));
                }
                _ => {}
            }
            return Err(FileError::Database(error));
        }
    };

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
