use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, FromRow)]
struct PermissionApiRow {
    code: String,
    path_pattern: String,
}

fn is_dynamic_segment(segment: &str) -> bool {
    (segment.starts_with('{') && segment.ends_with('}'))
        || (segment.starts_with(':') && segment.len() > 1)
}

fn normalized_path_matches(left: &str, right: &str) -> bool {
    left.trim_matches('/') == right.trim_matches('/')
}

fn normalize_method(method: &str) -> String {
    method.to_ascii_uppercase()
}

fn path_pattern_is_exact(pattern: &str) -> bool {
    !pattern.trim_matches('/').split('/').any(is_dynamic_segment)
}

pub fn path_pattern_matches(pattern: &str, path: &str) -> bool {
    let pattern_parts = pattern.trim_matches('/').split('/').collect::<Vec<_>>();
    let path_parts = path.trim_matches('/').split('/').collect::<Vec<_>>();

    if pattern_parts.len() != path_parts.len() {
        return false;
    }

    pattern_parts
        .iter()
        .zip(path_parts.iter())
        .all(|(left, right)| is_dynamic_segment(left) || left == right)
}

fn select_permission_for_path(rows: &[PermissionApiRow], path: &str) -> Option<String> {
    rows.iter()
        .find(|row| {
            path_pattern_is_exact(&row.path_pattern)
                && normalized_path_matches(&row.path_pattern, path)
        })
        .or_else(|| {
            rows.iter()
                .find(|row| path_pattern_matches(&row.path_pattern, path))
        })
        .map(|row| row.code.clone())
}

pub async fn resolve_required_permission(
    pool: &PgPool,
    method: &str,
    path: &str,
) -> Result<Option<String>, sqlx::Error> {
    let method = normalize_method(method);
    let rows = sqlx::query_as::<_, PermissionApiRow>(
        r#"
        SELECT p.code, pa.path_pattern
        FROM sys_permission_apis pa
        JOIN sys_permissions p ON p.id = pa.permission_id
        WHERE pa.method = $1
          AND p.status = 'enabled'
        "#,
    )
    .bind(method)
    .fetch_all(pool)
    .await?;

    Ok(select_permission_for_path(&rows, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_pattern_matches_static_paths() {
        assert!(path_pattern_matches("/api/users", "/api/users"));
        assert!(!path_pattern_matches("/api/users", "/api/roles"));
    }

    #[test]
    fn path_pattern_matches_brace_dynamic_segments() {
        assert!(path_pattern_matches("/api/users/{id}", "/api/users/42"));
        assert!(!path_pattern_matches(
            "/api/users/{id}",
            "/api/users/42/settings"
        ));
    }

    #[test]
    fn path_pattern_matches_colon_dynamic_segments() {
        assert!(path_pattern_matches("/api/users/:id", "/api/users/42"));
    }

    #[test]
    fn path_pattern_is_exact_rejects_dynamic_segments() {
        assert!(path_pattern_is_exact("/api/routes/batch"));
        assert!(!path_pattern_is_exact("/api/routes/{id}"));
        assert!(!path_pattern_is_exact("/api/routes/:id"));
    }

    #[test]
    fn select_permission_for_path_prefers_exact_match_over_dynamic_match() {
        let rows = vec![
            PermissionApiRow {
                code: "system:route:detail".to_string(),
                path_pattern: "/api/routes/{id}".to_string(),
            },
            PermissionApiRow {
                code: "system:route:batch".to_string(),
                path_pattern: "/api/routes/batch".to_string(),
            },
        ];

        assert_eq!(
            select_permission_for_path(&rows, "/api/routes/batch"),
            Some("system:route:batch".to_string())
        );
    }

    #[test]
    fn normalize_method_uppercases_method() {
        assert_eq!(normalize_method("get"), "GET");
        assert_eq!(normalize_method("Post"), "POST");
    }
}
