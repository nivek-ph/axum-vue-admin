#[derive(Debug, thiserror::Error)]
pub enum ParameterError {
    #[error("parameter storage operation failed")]
    Database(#[from] sqlx::Error),
}
