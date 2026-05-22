use std::borrow::Cow;

use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    /// Application HTTP error: status, stable machine code, human-facing message.
    #[error("{2}")]
    Http(StatusCode, Cow<'static, str>, String),

    #[error("invalid request body")]
    JsonRejection(#[from] JsonRejection),

    #[error("invalid json: `{0}`")]
    InvalidJson(#[from] serde_json::Error),

    #[error("internal server error")]
    Storage(#[from] sqlx::Error),

    #[error("internal server error")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("internal server error")]
    Any(#[from] anyhow::Error),
}

impl ErrorKind {
    pub(super) fn status_code(status: StatusCode) -> Cow<'static, str> {
        status
            .canonical_reason()
            .map(|reason| reason.to_ascii_uppercase().replace(' ', "_").into())
            .unwrap_or_else(|| status.as_u16().to_string().into())
    }

    pub fn http(
        status: StatusCode,
        code: impl Into<Cow<'static, str>>,
        message: impl Into<String>,
    ) -> Self {
        Self::Http(status, code.into(), message.into())
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::Http(status, _, _) => *status,
            Self::JsonRejection(_) | Self::InvalidJson(_) => StatusCode::BAD_REQUEST,
            Self::Storage(_) | Self::Migration(_) | Self::Any(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub fn code(&self) -> Cow<'static, str> {
        match self {
            Self::Http(_, code, _) => code.clone(),
            Self::Storage(_) | Self::Migration(_) | Self::Any(_) => {
                Self::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::JsonRejection(_) | Self::InvalidJson(_) => {
                Self::status_code(StatusCode::BAD_REQUEST)
            }
        }
    }
}
