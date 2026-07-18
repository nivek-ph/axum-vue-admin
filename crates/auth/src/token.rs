use redis::{AsyncCommands, aio::MultiplexedConnection};
use uuid::Uuid;

use crate::jwt::{Claims, JwtService};

const SESSION_KEY_PREFIX: &str = "auth:login-session:";
const SESSION_TTL_SECONDS: u64 = 24 * 60 * 60 + 1;

#[derive(Debug, thiserror::Error)]
pub enum TokenIssueError {
    #[error("token signing failed")]
    Signing(#[source] jsonwebtoken::errors::Error),
    #[error("token session store is unavailable")]
    StoreUnavailable,
    #[error("token session store operation failed")]
    Store(#[from] redis::RedisError),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenSessionError {
    #[error("token invalid")]
    Invalid(#[source] jsonwebtoken::errors::Error),
    #[error("token session invalid")]
    SessionInvalid,
    #[error("token session store is unavailable")]
    StoreUnavailable,
    #[error("token session store operation failed")]
    Store(#[from] redis::RedisError),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenRevokeError {
    #[error("token invalid")]
    Invalid(#[source] jsonwebtoken::errors::Error),
    #[error("token session store is unavailable")]
    StoreUnavailable,
    #[error("token session store operation failed")]
    Store(#[from] redis::RedisError),
}

#[derive(Clone)]
pub struct TokenService {
    jwt_service: JwtService,
    redis_connection: Option<MultiplexedConnection>,
}

impl TokenService {
    pub fn new(jwt_secret: &str, redis_connection: MultiplexedConnection) -> Self {
        Self {
            jwt_service: JwtService::new(jwt_secret),
            redis_connection: Some(redis_connection),
        }
    }

    pub fn without_session_store(jwt_secret: &str) -> Self {
        Self {
            jwt_service: JwtService::new(jwt_secret),
            redis_connection: None,
        }
    }

    pub async fn issue(&self, user_id: i64, username: &str) -> Result<String, TokenIssueError> {
        let session_id = Uuid::new_v4().to_string();
        let token = self
            .jwt_service
            .issue_token(user_id, username, &session_id)
            .map_err(TokenIssueError::Signing)?;
        let mut redis = self
            .redis_connection
            .clone()
            .ok_or(TokenIssueError::StoreUnavailable)?;
        let _: () = redis
            .set_ex(session_key(&session_id), user_id, SESSION_TTL_SECONDS)
            .await?;
        Ok(token)
    }

    pub async fn decode_active(&self, token: &str) -> Result<Claims, TokenSessionError> {
        let claims = self
            .jwt_service
            .decode_token(token)
            .map_err(TokenSessionError::Invalid)?;
        let mut redis = self
            .redis_connection
            .clone()
            .ok_or(TokenSessionError::StoreUnavailable)?;
        let session_user_id: Option<i64> = redis.get(session_key(&claims.sid)).await?;
        if session_user_id != Some(claims.user_id) {
            return Err(TokenSessionError::SessionInvalid);
        }
        Ok(claims)
    }

    pub async fn revoke(&self, token: &str) -> Result<(), TokenRevokeError> {
        let claims = self
            .jwt_service
            .decode_token(token)
            .map_err(TokenRevokeError::Invalid)?;
        let mut redis = self
            .redis_connection
            .clone()
            .ok_or(TokenRevokeError::StoreUnavailable)?;
        let _: usize = redis.del(session_key(&claims.sid)).await?;
        Ok(())
    }
}

#[inline]
fn session_key(session_id: &str) -> String {
    format!("{SESSION_KEY_PREFIX}{session_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn issuing_without_session_store_fails_closed() {
        let error = TokenService::without_session_store("test-secret")
            .issue(1, "admin")
            .await
            .expect_err("issuing without a session store should fail");

        assert!(matches!(error, TokenIssueError::StoreUnavailable));
    }

    #[tokio::test]
    async fn login_sessions_are_independent_and_revocable() {
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());
        let redis = redis::Client::open(redis_url)
            .expect("Redis test client should construct")
            .get_multiplexed_async_connection()
            .await
            .expect("Redis test connection should open");
        let tokens = TokenService::new("test-secret", redis);

        let first = tokens
            .issue(1, "admin")
            .await
            .expect("first login session should be issued");
        let second = tokens
            .issue(1, "admin")
            .await
            .expect("second login session should be issued");
        let first_claims = tokens
            .decode_active(&first)
            .await
            .expect("first login session should be active");
        let second_claims = tokens
            .decode_active(&second)
            .await
            .expect("second login session should be active");
        assert_ne!(first_claims.sid, second_claims.sid);

        tokens
            .revoke(&first)
            .await
            .expect("first login session should be revoked");
        let error = tokens
            .decode_active(&first)
            .await
            .expect_err("revoked login session should be rejected");
        assert!(matches!(error, TokenSessionError::SessionInvalid));
        tokens
            .decode_active(&second)
            .await
            .expect("second login session should remain active");

        tokens
            .revoke(&second)
            .await
            .expect("second login session should be cleaned up");
    }
}
