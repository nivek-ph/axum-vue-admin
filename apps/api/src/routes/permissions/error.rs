use admin_httpz::{AppError, ErrorSpec};
use iam::permissions::PermissionError;

const NOT_FOUND: ErrorSpec = ErrorSpec::not_found("PERMISSION_NOT_FOUND", "permission not found");
const INVALID_CODE: ErrorSpec =
    ErrorSpec::validation("PERMISSION_INVALID_CODE", "invalid permission code");
const DB_FAILED: ErrorSpec =
    ErrorSpec::internal("PERMISSION_DB_FAILED", "permission operation failed");

pub fn map_error(error: PermissionError) -> AppError {
    match error {
        PermissionError::Database(source) => DB_FAILED.into_error().with_source(source),
        PermissionError::NotFound => NOT_FOUND.into(),
        PermissionError::InvalidCode => INVALID_CODE.into(),
    }
}
