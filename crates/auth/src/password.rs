use argon2::Argon2;
use password_hash::{PasswordHasher, PasswordVerifier};

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("authentication failed")]
    AuthFailed,
    #[error("password hash invalid")]
    PasswordHashInvalid(#[from] password_hash::Error),
}

#[derive(Debug, Clone, Default)]
pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> Result<String, PasswordError> {
        Ok(Argon2::default()
            .hash_password(password.as_bytes())?
            .to_string())
    }

    pub fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, PasswordError> {
        match Argon2::default().verify_password(password.as_bytes(), password_hash) {
            Ok(()) => Ok(true),
            Err(password_hash::Error::PasswordInvalid) => Ok(false),
            Err(error) => Err(PasswordError::PasswordHashInvalid(error)),
        }
    }
}
