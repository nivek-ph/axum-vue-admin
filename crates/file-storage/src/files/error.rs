#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
