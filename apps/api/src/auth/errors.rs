use admin_httpz::ErrorSpec;

pub const LOGIN_REQUIRED: ErrorSpec = ErrorSpec::unauthorized("LOGIN_REQUIRED", "请先登录");
pub const TOKEN_INVALID: ErrorSpec = ErrorSpec::unauthorized("TOKEN_INVALID", "登录已失效");
pub const SESSION_INVALID: ErrorSpec = ErrorSpec::unauthorized("SESSION_INVALID", "登录已失效");

/// Unexpected failure while resolving the authenticated user after a valid JWT (middleware).
pub const AUTH_RESOLVE_FAILED: ErrorSpec =
    ErrorSpec::internal("AUTH_RESOLVE_FAILED", "用户鉴权失败");

/// Login could not complete due to an internal failure (logged; external message is generic).
pub const LOGIN_OPERATION_FAILED: ErrorSpec =
    ErrorSpec::internal("LOGIN_OPERATION_FAILED", "登录失败");
