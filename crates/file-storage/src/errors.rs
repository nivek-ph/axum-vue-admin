use admin_httpz::ErrorSpec;

pub const FILE_DB_FAILED: ErrorSpec =
    ErrorSpec::internal("FILE_DB_FAILED", "file operation failed");
pub const FILE_IO_FAILED: ErrorSpec =
    ErrorSpec::internal("FILE_IO_FAILED", "file operation failed");
