#[derive(Debug, thiserror::Error)]
pub enum CategoryError {
    #[error("file category storage operation failed")]
    Database(#[from] sqlx::Error),
}
