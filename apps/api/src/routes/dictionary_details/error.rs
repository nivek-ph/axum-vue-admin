use admin_httpz::{AppError, ErrorSpec};

pub const ID_REQUIRED: ErrorSpec = ErrorSpec::bad_request("ID_REQUIRED", "id is required");
const OPERATION_FAILED: ErrorSpec =
    ErrorSpec::internal("INTERNAL_SERVER_ERROR", "internal server error");

pub fn map_error(error: metadata::dictionaries::DictionaryError) -> AppError {
    match error {
        metadata::dictionaries::DictionaryError::Database(source) => {
            OPERATION_FAILED.into_error().with_source(source)
        }
    }
}
