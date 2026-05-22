use admin_httpz::ErrorSpec;

pub mod authority {
    use super::*;

    pub const AUTHORITY_EXISTS: ErrorSpec = ErrorSpec::conflict("AUTHORITY_EXISTS", "角色已存在");
    pub const ROOT_AUTHORITY_IMMUTABLE: ErrorSpec =
        ErrorSpec::failed_precondition("ROOT_AUTHORITY_IMMUTABLE", "默认角色不可删除");
    pub const AUTHORITY_NOT_FOUND: ErrorSpec =
        ErrorSpec::not_found("AUTHORITY_NOT_FOUND", "角色不存在");
    pub const AUTHORITY_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("AUTHORITY_DB_FAILED", "角色操作失败");
}

pub mod menu {
    use super::*;

    pub const MENU_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("MENU_NOT_FOUND", "菜单不存在");
    pub const MENU_INVALID_PAYLOAD: ErrorSpec =
        ErrorSpec::validation("MENU_INVALID_PAYLOAD", "菜单数据格式错误");
    pub const MENU_DB_FAILED: ErrorSpec = ErrorSpec::internal("MENU_DB_FAILED", "菜单操作失败");
}

pub mod api_registry {
    use super::*;

    pub const API_EXISTS: ErrorSpec = ErrorSpec::conflict("API_EXISTS", "API 已存在");
    pub const API_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("API_NOT_FOUND", "API 不存在");
    pub const API_DB_FAILED: ErrorSpec = ErrorSpec::internal("API_DB_FAILED", "API 操作失败");
}

pub mod users {
    use super::*;

    pub const INVALID_CREDENTIALS: ErrorSpec =
        ErrorSpec::unauthorized("INVALID_CREDENTIALS", "用户名或密码错误");
    pub const USER_DISABLED: ErrorSpec = ErrorSpec::forbidden("USER_DISABLED", "用户已被禁用");
    pub const USER_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("USER_NOT_FOUND", "用户不存在");
    pub const USER_ALREADY_EXISTS: ErrorSpec =
        ErrorSpec::conflict("USER_ALREADY_EXISTS", "用户已存在");
    pub const INVALID_PASSWORD: ErrorSpec = ErrorSpec::bad_request("INVALID_PASSWORD", "密码错误");
    pub const USER_OPERATION_FAILED: ErrorSpec =
        ErrorSpec::internal("USER_OPERATION_FAILED", "用户操作失败");
}

pub mod params {
    use super::*;

    pub const PARAM_JSON_ENCODE_FAILED: ErrorSpec =
        ErrorSpec::internal("PARAM_JSON_ENCODE_FAILED", "参数序列化失败");
}
