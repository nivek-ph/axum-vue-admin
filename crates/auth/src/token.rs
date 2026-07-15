use std::time::{Duration, SystemTime, UNIX_EPOCH};

use redis::{AsyncCommands, aio::MultiplexedConnection};
use sha2::{Digest, Sha256};

use crate::jwt::{Claims, JwtService};

const REDIS_HASH_KEY: &str = "auth:revoked-tokens";

#[derive(Debug, thiserror::Error)]
pub enum TokenIssueError {
    #[error("token signing failed")]
    Signing(#[source] jsonwebtoken::errors::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenSessionError {
    #[error("token invalid")]
    Invalid(#[source] jsonwebtoken::errors::Error),
    #[error("token revoked")]
    Revoked,
    #[error("token revocation store is unavailable")]
    StoreUnavailable,
    #[error("token revocation store operation failed")]
    Store(#[from] redis::RedisError),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenRevokeError {
    #[error("token invalid")]
    Invalid(#[source] jsonwebtoken::errors::Error),
    #[error("token revocation store is unavailable")]
    StoreUnavailable,
    #[error("token revocation store operation failed")]
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

    pub fn without_revocation_store(jwt_secret: &str) -> Self {
        Self {
            jwt_service: JwtService::new(jwt_secret),
            redis_connection: None,
        }
    }

    pub fn issue(&self, user_id: i64, username: &str) -> Result<String, TokenIssueError> {
        self.jwt_service
            .issue_token(user_id, username)
            .map_err(TokenIssueError::Signing)
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
        let revoked: bool = redis
            .hexists(REDIS_HASH_KEY, redis_hash_field_for_token(token))
            .await?;
        if revoked {
            return Err(TokenSessionError::Revoked);
        }
        Ok(claims)
    }

    pub async fn revoke(&self, token: &str) -> Result<(), TokenRevokeError> {
        let claims = self
            .jwt_service
            .decode_token(token)
            .map_err(TokenRevokeError::Invalid)?;
        let now_epoch = current_epoch_seconds();
        let Some(ttl) = redis_ttl_seconds(claims.exp, now_epoch) else {
            return Ok(());
        };

        let mut redis = self
            .redis_connection
            .clone()
            .ok_or(TokenRevokeError::StoreUnavailable)?;
        let field = redis_hash_field_for_token(token);
        set_revoked_token_field(&mut redis, &field, now_epoch.to_string(), ttl).await?;
        Ok(())
    }
}

fn current_epoch_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs() as i64
}

#[inline]
fn redis_hash_field_for_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

#[inline]
fn redis_ttl_seconds(exp: usize, now_epoch: i64) -> Option<u64> {
    let remaining = exp as i64 - now_epoch;
    (remaining > 0).then_some((remaining + 1) as u64)
}

async fn set_revoked_token_field(
    redis: &mut MultiplexedConnection,
    field: &str,
    value: String,
    ttl: u64,
) -> Result<(), redis::RedisError> {
    let _: (usize, Vec<i64>) = redis::pipe()
        .hset(REDIS_HASH_KEY, field, value)
        .cmd("HEXPIRE")
        .arg(REDIS_HASH_KEY)
        .arg(ttl)
        .arg("FIELDS")
        .arg(1)
        .arg(field)
        .query_async(redis)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_token_for_revocation_storage() {
        let first = redis_hash_field_for_token("token-one");
        let second = redis_hash_field_for_token("token-two");
        assert_eq!(first.len(), 64);
        assert_ne!(first, second);
        assert!(first.chars().all(|ch| ch.is_ascii_hexdigit()));
    }

    #[test]
    fn ttl_exceeds_remaining_token_lifetime() {
        assert_eq!(redis_ttl_seconds(101, 100), Some(2));
        assert_eq!(redis_ttl_seconds(100, 100), None);
        assert_eq!(redis_ttl_seconds(99, 100), None);
    }
}
