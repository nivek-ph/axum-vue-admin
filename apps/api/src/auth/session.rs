use std::time::{Duration, SystemTime, UNIX_EPOCH};

use auth::{
    jwt::{Claims, JwtService},
    password::{AuthError, PasswordService},
};
use redis::{AsyncCommands, aio::MultiplexedConnection};
use sha2::{Digest, Sha256};

const REDIS_HASH_KEY: &str = "auth:revoked-tokens";

#[derive(Debug, thiserror::Error)]
pub enum AuthSessionError {
    #[error("{0}")]
    Auth(#[from] AuthError),
    #[error("token revoked")]
    Revoked,
    #[error("redis revoke store is unavailable")]
    RevocationStoreUnavailable,
    #[error("{0}")]
    Redis(#[from] redis::RedisError),
}

#[derive(Clone)]
pub struct AuthSessionService {
    jwt_service: JwtService,
    redis_connection: Option<MultiplexedConnection>,
}

impl AuthSessionService {
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

    pub async fn login(
        &self,
        pool: &sqlx::PgPool,
        password_service: &PasswordService,
        payload: system::users::LoginRequest,
    ) -> Result<system::users::LoginResult, system::users::LoginError> {
        system::users::login(pool, password_service, &self.jwt_service, payload).await
    }

    pub async fn decode_active_token(&self, token: &str) -> Result<Claims, AuthSessionError> {
        let claims = self.decode_token(token)?;
        if self.is_token_revoked(token).await? {
            return Err(AuthSessionError::Revoked);
        }
        Ok(claims)
    }

    pub async fn revoke_token(&self, token: &str) -> Result<(), AuthSessionError> {
        let claims = self.decode_token(token)?;
        let now_epoch = current_epoch_seconds();
        let Some(ttl) = redis_ttl_seconds(claims.exp, now_epoch) else {
            return Ok(());
        };

        let mut redis = self.redis_connection()?;
        let field = redis_hash_field_for_token(token);
        set_revoked_token_field(&mut redis, &field, redis_revoke_value(now_epoch), ttl).await?;

        Ok(())
    }

    pub async fn is_token_revoked(&self, token: &str) -> Result<bool, AuthSessionError> {
        let mut redis = self.redis_connection()?;
        let revoked: bool = redis
            .hexists(REDIS_HASH_KEY, redis_hash_field_for_token(token))
            .await?;
        Ok(revoked)
    }

    fn redis_connection(&self) -> Result<MultiplexedConnection, AuthSessionError> {
        self.redis_connection
            .clone()
            .ok_or(AuthSessionError::RevocationStoreUnavailable)
    }

    fn decode_token(&self, token: &str) -> Result<Claims, AuthError> {
        self.jwt_service.decode_token(token)
    }
}

fn current_epoch_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs() as i64
}

fn token_hash_for_storage(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

fn redis_hash_field_for_token(token: &str) -> String {
    token_hash_for_storage(token)
}

fn redis_ttl_seconds(exp: usize, now_epoch: i64) -> Option<u64> {
    let remaining = exp as i64 - now_epoch;
    if remaining <= 0 {
        return None;
    }
    Some((remaining + 1) as u64)
}

fn redis_revoke_value(now_epoch: i64) -> String {
    now_epoch.to_string()
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

    #[test]
    fn revocation_value_is_current_epoch_second() {
        assert_eq!(redis_revoke_value(12345), "12345");
    }
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
