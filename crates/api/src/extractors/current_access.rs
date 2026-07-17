use axum::{extract::FromRequestParts, http::request::Parts};
use iam::access::AccessSnapshot;

use crate::{AppError, mappings::LOGIN_REQUIRED};

#[derive(Debug, Clone)]
pub(crate) struct CurrentAccess(pub AccessSnapshot);

impl<S> FromRequestParts<S> for CurrentAccess
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .remove::<Self>()
            .ok_or(LOGIN_REQUIRED.into())
    }
}
