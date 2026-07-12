use admin_httpz::{AppError, ErrorSpec};
use iam::apis::ApiError;

const EXISTS: ErrorSpec = ErrorSpec::conflict("API_EXISTS", "API already exists");
const NOT_FOUND: ErrorSpec = ErrorSpec::not_found("API_NOT_FOUND", "API not found");
const DB_FAILED: ErrorSpec = ErrorSpec::internal("API_DB_FAILED", "API operation failed");

pub fn map_error(error: ApiError) -> AppError {
    match error {
        ApiError::AlreadyExists => EXISTS.into(),
        ApiError::NotFound => NOT_FOUND.into(),
        ApiError::Database(source) => DB_FAILED.into_error().with_source(source),
    }
}
