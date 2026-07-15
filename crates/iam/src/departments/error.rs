#[derive(Debug, thiserror::Error)]
pub enum DeptError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("invalid department parent")]
    InvalidParent,
    #[error(transparent)]
    Access(#[from] crate::access::AccessError),
}
