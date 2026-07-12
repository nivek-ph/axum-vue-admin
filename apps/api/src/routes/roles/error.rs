use admin_httpz::{AppError, ErrorSpec};
use iam::roles::RoleError;

const ROLE_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("ROLE_NOT_FOUND", "role not found");
const ROLE_IMMUTABLE: ErrorSpec =
    ErrorSpec::failed_precondition("ROLE_IMMUTABLE", "system role cannot be deleted");
const ROLE_DB_FAILED: ErrorSpec = ErrorSpec::internal("ROLE_DB_FAILED", "role operation failed");

pub fn map_error(error: RoleError) -> AppError {
    match error {
        RoleError::Database(source) => ROLE_DB_FAILED.into_error().with_source(source),
        RoleError::NotFound => ROLE_NOT_FOUND.into(),
        RoleError::Immutable => ROLE_IMMUTABLE.into(),
    }
}
