use admin_httpz::ErrorSpec;

pub const FILE_DB_FAILED: ErrorSpec = ErrorSpec::internal("FILE_DB_FAILED", "文件操作失败");
pub const FILE_IO_FAILED: ErrorSpec = ErrorSpec::internal("FILE_IO_FAILED", "文件操作失败");
