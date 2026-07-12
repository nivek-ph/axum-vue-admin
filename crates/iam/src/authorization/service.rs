use sqlx::PgPool;

use super::AuthorizationError;
use crate::{api_permissions, permissions, roles, users};

#[derive(Clone)]
pub struct AuthorizationService {
    pool: PgPool,
}

impl AuthorizationService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn resolve_user(
        &self,
        user_id: i64,
    ) -> Result<users::AuthenticatedUser, users::LoginError> {
        users::service::load_authenticated_user(&self.pool, user_id).await
    }

    pub async fn has_super_admin_role(&self, user_id: i64) -> Result<bool, AuthorizationError> {
        Ok(roles::user_has_role_code(&self.pool, user_id, "super_admin").await?)
    }

    pub async fn required_permission(
        &self,
        method: &str,
        path: &str,
    ) -> Result<Option<String>, AuthorizationError> {
        Ok(api_permissions::resolve_required_permission(&self.pool, method, path).await?)
    }

    pub async fn is_allowed(
        &self,
        user_id: i64,
        permission: &str,
    ) -> Result<bool, AuthorizationError> {
        Ok(permissions::user_has_permission(&self.pool, user_id, permission).await?)
    }
}
