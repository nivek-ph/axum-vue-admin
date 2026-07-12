#[derive(Debug, thiserror::Error)]
pub enum AuthorizationError {
    #[error("authorization storage operation failed")]
    Database(#[from] sqlx::Error),
}
