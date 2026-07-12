#[derive(Debug, thiserror::Error)]
pub enum LoginLogError {
    #[error("login log storage operation failed")]
    Database(#[from] sqlx::Error),
}
