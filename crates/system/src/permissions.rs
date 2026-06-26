use sqlx::PgPool;

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
}
