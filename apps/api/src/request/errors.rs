use admin_httpz::ErrorSpec;

pub const ID_REQUIRED: ErrorSpec = ErrorSpec::bad_request("ID_REQUIRED", "缺少ID");
pub const AUTHORITY_ID_REQUIRED: ErrorSpec =
    ErrorSpec::bad_request("AUTHORITY_ID_REQUIRED", "缺少authorityId");
pub const MULTIPART_FIELD_FAILED: ErrorSpec =
    ErrorSpec::bad_request("MULTIPART_FIELD_FAILED", "上传内容读取失败");
