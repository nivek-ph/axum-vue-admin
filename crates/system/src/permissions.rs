use crate::errors;
use admin_httpz::AppError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct PermissionSummary {
    pub id: i64,
    pub module_key: String,
    pub resource: String,
    pub action: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub permission_type: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionPayload {
    #[serde(alias = "moduleKey")]
    pub module_key: String,
    pub resource: String,
    pub action: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub permission_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq, Eq)]
pub struct PermissionApiBinding {
    pub method: String,
    #[serde(alias = "pathPattern")]
    pub path_pattern: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PermissionApisPayload {
    pub apis: Vec<PermissionApiBinding>,
}

#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("permission not found")]
    NotFound,
    #[error("invalid permission code")]
    InvalidCode,
}

impl From<PermissionError> for AppError {
    fn from(error: PermissionError) -> Self {
        match error {
            PermissionError::Database(error) => errors::permissions::PERMISSION_DB_FAILED
                .into_error()
                .with_source(error),
            PermissionError::NotFound => errors::permissions::PERMISSION_NOT_FOUND.into(),
            PermissionError::InvalidCode => errors::permissions::PERMISSION_INVALID_CODE.into(),
        }
    }
}

pub fn is_valid_permission_code(code: &str) -> bool {
    let parts = code.split(':').collect::<Vec<_>>();
    parts.len() == 3
        && parts.iter().all(|part| {
            !part.is_empty()
                && part.chars().all(|ch| {
                    ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-'
                })
        })
}

pub async fn user_has_permission(
    pool: &PgPool,
    user_id: i64,
    permission_code: &str,
) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM sys_user_roles ur
            JOIN sys_roles r ON r.id = ur.role_id
            JOIN sys_role_permissions rp ON rp.role_id = r.id
            JOIN sys_permissions p ON p.id = rp.permission_id
            WHERE ur.user_id = $1
              AND p.code = $2
              AND r.status = 'enabled'
              AND p.status = 'enabled'
        )
        "#,
    )
    .bind(user_id)
    .bind(permission_code)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn list(pool: &PgPool) -> Result<Vec<PermissionSummary>, sqlx::Error> {
    sqlx::query_as::<_, PermissionSummary>(
        r#"
        select id, module_key, resource, action, code, name, type as permission_type, status
        from sys_permissions
        order by module_key, resource, action, id
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find(pool: &PgPool, id: i64) -> Result<Option<PermissionSummary>, sqlx::Error> {
    sqlx::query_as::<_, PermissionSummary>(
        r#"
        select id, module_key, resource, action, code, name, type as permission_type, status
        from sys_permissions
        where id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    payload: PermissionPayload,
) -> Result<PermissionSummary, PermissionError> {
    validate_permission_code(&payload.code)?;

    let permission = sqlx::query_as::<_, PermissionSummary>(
        r#"
        insert into sys_permissions (module_key, resource, action, code, name, type, status)
        values ($1, $2, $3, $4, $5, $6, $7)
        returning id, module_key, resource, action, code, name, type as permission_type, status
        "#,
    )
    .bind(payload.module_key)
    .bind(payload.resource)
    .bind(payload.action)
    .bind(payload.code)
    .bind(payload.name)
    .bind(
        payload
            .permission_type
            .unwrap_or_else(|| "action".to_string()),
    )
    .bind(payload.status.unwrap_or_else(|| "enabled".to_string()))
    .fetch_one(pool)
    .await?;

    Ok(permission)
}

pub async fn update(
    pool: &PgPool,
    id: i64,
    payload: PermissionPayload,
) -> Result<PermissionSummary, PermissionError> {
    validate_permission_code(&payload.code)?;

    let permission = sqlx::query_as::<_, PermissionSummary>(
        r#"
        update sys_permissions
        set module_key = $1,
            resource = $2,
            action = $3,
            code = $4,
            name = $5,
            type = coalesce($6, type),
            status = coalesce($7, status),
            updated_at = now()
        where id = $8
        returning id, module_key, resource, action, code, name, type as permission_type, status
        "#,
    )
    .bind(payload.module_key)
    .bind(payload.resource)
    .bind(payload.action)
    .bind(payload.code)
    .bind(payload.name)
    .bind(payload.permission_type)
    .bind(payload.status)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    permission.ok_or(PermissionError::NotFound)
}

pub async fn delete(pool: &PgPool, id: i64) -> Result<(), PermissionError> {
    ensure_exists(pool, id).await?;
    sqlx::query("delete from sys_permissions where id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn api_bindings(
    pool: &PgPool,
    permission_id: i64,
) -> Result<Vec<PermissionApiBinding>, PermissionError> {
    ensure_exists(pool, permission_id).await?;
    let apis = sqlx::query_as::<_, PermissionApiBinding>(
        r#"
        select method, path_pattern
        from sys_permission_apis
        where permission_id = $1
        order by method, path_pattern
        "#,
    )
    .bind(permission_id)
    .fetch_all(pool)
    .await?;

    Ok(apis)
}

pub async fn set_api_bindings(
    pool: &PgPool,
    permission_id: i64,
    apis: Vec<PermissionApiBinding>,
) -> Result<(), PermissionError> {
    ensure_exists(pool, permission_id).await?;

    let mut tx = pool.begin().await?;
    sqlx::query("delete from sys_permission_apis where permission_id = $1")
        .bind(permission_id)
        .execute(&mut *tx)
        .await?;

    for api in apis {
        sqlx::query(
            r#"
            insert into sys_permission_apis (permission_id, method, path_pattern)
            values ($1, $2, $3)
            on conflict do nothing
            "#,
        )
        .bind(permission_id)
        .bind(api.method.to_ascii_uppercase())
        .bind(api.path_pattern)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

fn validate_permission_code(code: &str) -> Result<(), PermissionError> {
    if is_valid_permission_code(code) {
        Ok(())
    } else {
        Err(PermissionError::InvalidCode)
    }
}

async fn ensure_exists(pool: &PgPool, id: i64) -> Result<(), PermissionError> {
    find(pool, id)
        .await?
        .map(|_| ())
        .ok_or(PermissionError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_permission_code_uses_module_resource_action() {
        assert!(is_valid_permission_code("system:user:create"));
        assert!(is_valid_permission_code("ai_app:agent:debug"));
    }

    #[test]
    fn invalid_permission_code_rejects_bad_shapes() {
        assert!(!is_valid_permission_code("system:user"));
        assert!(!is_valid_permission_code("system:user:create:extra"));
        assert!(!is_valid_permission_code("System:user:create"));
        assert!(!is_valid_permission_code("system::create"));
    }

    #[test]
    fn api_binding_method_can_be_normalized_before_storage() {
        let binding = PermissionApiBinding {
            method: "get".to_string(),
            path_pattern: "/api/users".to_string(),
        };

        assert_eq!(binding.method.to_ascii_uppercase(), "GET");
    }
}
