use auth::password::AuthError;

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("invalid username or password")]
    InvalidCredentials,
    #[error("user is disabled")]
    Disabled,
    #[error("user not found")]
    UserNotFound,
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("invalid password")]
    InvalidPassword,
    #[error("{0}")]
    Auth(#[from] AuthError),
    #[error("{0}")]
    Database(#[from] sqlx::Error),
}
