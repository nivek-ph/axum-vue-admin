#[derive(Debug, thiserror::Error)]
pub enum OperationLogError {
    #[error("operation log storage operation failed")]
    Database(#[from] sqlx::Error),
}
