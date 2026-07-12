#[derive(Debug, thiserror::Error)]
pub enum PermissionError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("permission not found")]
    NotFound,
    #[error("invalid permission code")]
    InvalidCode,
}
