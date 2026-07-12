#[derive(Debug, thiserror::Error)]
pub enum DictionaryError {
    #[error("dictionary storage operation failed")]
    Database(#[from] sqlx::Error),
}
