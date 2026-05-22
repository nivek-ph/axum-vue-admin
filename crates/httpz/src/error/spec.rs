use axum::http::StatusCode;

use super::{AppError, AppResult, ErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorSpec {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
}

impl ErrorSpec {
    pub const fn new(status: StatusCode, code: &'static str, message: &'static str) -> Self {
        Self {
            status,
            code,
            message,
        }
    }

    pub const fn bad_request(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    pub const fn validation(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    pub const fn unauthorized(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, code, message)
    }

    pub const fn forbidden(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::FORBIDDEN, code, message)
    }

    pub const fn not_found(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub const fn conflict(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::CONFLICT, code, message)
    }

    pub const fn failed_precondition(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::PRECONDITION_FAILED, code, message)
    }

    pub const fn internal(code: &'static str, message: &'static str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message)
    }

    pub fn into_error(self) -> AppError {
        ErrorKind::http(self.status, self.code, self.message).into()
    }
}

impl From<ErrorSpec> for ErrorKind {
    fn from(spec: ErrorSpec) -> Self {
        Self::http(spec.status, spec.code, spec.message)
    }
}

pub trait ErrorSpecExt {
    fn err<T>(self) -> AppResult<T>;
}

impl ErrorSpecExt for ErrorSpec {
    fn err<T>(self) -> AppResult<T> {
        Err(self.into_error())
    }
}

pub trait OptionAppExt<T> {
    fn ok_or_spec(self, spec: ErrorSpec) -> AppResult<T>;
}

impl<T> OptionAppExt<T> for Option<T> {
    fn ok_or_spec(self, spec: ErrorSpec) -> AppResult<T> {
        self.ok_or_else(|| spec.into_error())
    }
}
