#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("API already exists")]
    AlreadyExists,
    #[error("API not found")]
    NotFound,
    #[error("{0}")]
    Database(#[from] sqlx::Error),
}
