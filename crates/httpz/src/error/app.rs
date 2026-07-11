use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

use anyhow::Error as AnyError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing_error::SpanTrace;

use super::ErrorKind;
use crate::response::ApiErrorResponse;

#[derive(Debug)]
pub struct AppError {
    kind: ErrorKind,
    context: SpanTrace,
    source: Option<AnyError>,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.kind)?;
        Display::fmt(&self.context, f)
    }
}

impl StdError for AppError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let Some(error) = self.source.as_ref() {
            return Some(error.as_ref());
        }
        self.kind.source()
    }
}

impl<T> From<T> for AppError
where
    T: Into<ErrorKind>,
{
    fn from(value: T) -> Self {
        Self {
            kind: value.into(),
            context: SpanTrace::capture(),
            source: None,
        }
    }
}

impl AppError {
    pub fn new(
        status: StatusCode,
        code: impl Into<Cow<'static, str>>,
        message: impl Into<String>,
    ) -> Self {
        Self::custom(status, code, message)
    }

    pub fn custom(
        status: StatusCode,
        code: impl Into<Cow<'static, str>>,
        message: impl Into<String>,
    ) -> Self {
        ErrorKind::http(status, code, message).into()
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        let status = StatusCode::BAD_REQUEST;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::bad_request(message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        let status = StatusCode::UNAUTHORIZED;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn unauthenticated(message: impl Into<String>) -> Self {
        Self::unauthorized(message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        let status = StatusCode::FORBIDDEN;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        let status = StatusCode::NOT_FOUND;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        let status = StatusCode::CONFLICT;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn failed_precondition(message: impl Into<String>) -> Self {
        let status = StatusCode::PRECONDITION_FAILED;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        let status = StatusCode::SERVICE_UNAVAILABLE;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn internal(message: impl Into<String>) -> Self {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        ErrorKind::http(status, ErrorKind::status_code(status), message).into()
    }

    pub fn with_code(mut self, code: impl Into<Cow<'static, str>>) -> Self {
        let code_cow = code.into();
        self.kind = match self.kind {
            ErrorKind::Http(status, _, msg) => ErrorKind::Http(status, code_cow, msg),
            other => ErrorKind::Http(other.status(), code_cow, other.to_string()),
        };
        self
    }

    pub fn with_source(mut self, error: impl Into<AnyError>) -> Self {
        self.source = Some(error.into());
        self
    }

    pub fn from_error(status: StatusCode, error: impl Into<AnyError>) -> Self {
        let error = error.into();
        let msg = error.to_string();
        AppError::custom(status, ErrorKind::status_code(status), msg).with_source(error)
    }

    pub fn bad_request_error(error: impl Into<AnyError>) -> Self {
        Self::from_error(StatusCode::BAD_REQUEST, error)
    }

    pub fn validation_error(error: impl Into<AnyError>) -> Self {
        Self::bad_request_error(error)
    }

    pub fn unauthenticated_error(error: impl Into<AnyError>) -> Self {
        Self::from_error(StatusCode::UNAUTHORIZED, error)
    }

    pub fn internal_error(error: impl Into<AnyError>) -> Self {
        Self::from_error(StatusCode::INTERNAL_SERVER_ERROR, error)
    }

    pub fn status(&self) -> StatusCode {
        self.kind.status()
    }

    pub fn code(&self) -> Cow<'static, str> {
        self.kind.code()
    }

    pub fn message(&self) -> String {
        match &self.kind {
            ErrorKind::Http(_, _, msg) => msg.clone(),
            _ => self.kind.to_string(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn span_trace(&self) -> &SpanTrace {
        &self.context
    }
}

const INTERNAL_SERVER_ERROR: &str = "internal server error";

fn public_response_message(kind: &ErrorKind, diagnostic: &str) -> String {
    match kind {
        ErrorKind::Storage(_) | ErrorKind::Migration(_) | ErrorKind::Any(_) => {
            INTERNAL_SERVER_ERROR.to_string()
        }
        ErrorKind::Http(status, _, _) => {
            if *status == StatusCode::REQUEST_TIMEOUT {
                return "request timed out".to_string();
            }
            if *status == StatusCode::SERVICE_UNAVAILABLE {
                return "service unavailable".to_string();
            }
            if status.is_server_error() {
                return INTERNAL_SERVER_ERROR.to_string();
            }
            diagnostic.to_string()
        }
        _ => diagnostic.to_string(),
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let code = self.code().into_owned();
        let message = self.message();
        let public_message = public_response_message(&self.kind, &message);
        let span_trace = self.context;
        let kind = self.kind;
        let attach = self.source;

        if status.is_server_error() {
            tracing::error!(
                status = status.as_u16(),
                code = %code,
                message = %message,
                error = %kind,
                attach = ?attach,
                span_trace = %span_trace,
                "request failed",
            );
        } else {
            tracing::warn!(
                status = status.as_u16(),
                code = %code,
                message = %message,
                error = %kind,
                attach = ?attach,
                span_trace = %span_trace,
                "request failed",
            );
        }

        let body = ApiErrorResponse::new(code, public_message, None);
        (status, Json(body)).into_response()
    }
}
