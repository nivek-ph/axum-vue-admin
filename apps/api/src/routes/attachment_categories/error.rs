use admin_httpz::{AppError, ErrorSpec};

const DB_FAILED: ErrorSpec = ErrorSpec::internal("INTERNAL_SERVER_ERROR", "internal server error");

pub fn map_error(error: file_storage::categories::CategoryError) -> AppError {
    match error {
        file_storage::categories::CategoryError::Database(source) => {
            DB_FAILED.into_error().with_source(source)
        }
    }
}
