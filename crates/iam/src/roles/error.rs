use crate::access::{AccessError, CatalogError};

#[derive(Debug, thiserror::Error)]
pub enum RoleError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("role not found")]
    NotFound,
    #[error("system role cannot be deleted")]
    Immutable,
    #[error("role is assigned to users")]
    InUse,
    #[error(transparent)]
    Access(#[from] AccessError),
    #[error(transparent)]
    InvalidMenuAssignment(#[from] CatalogError),
}
