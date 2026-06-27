use admin_httpz::ErrorSpec;

pub mod authority {
    use super::*;

    pub const AUTHORITY_EXISTS: ErrorSpec =
        ErrorSpec::conflict("AUTHORITY_EXISTS", "role already exists");
    pub const ROOT_AUTHORITY_IMMUTABLE: ErrorSpec = ErrorSpec::failed_precondition(
        "ROOT_AUTHORITY_IMMUTABLE",
        "default role cannot be deleted",
    );
    pub const AUTHORITY_NOT_FOUND: ErrorSpec =
        ErrorSpec::not_found("AUTHORITY_NOT_FOUND", "role not found");
    pub const AUTHORITY_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("AUTHORITY_DB_FAILED", "role operation failed");
}

pub mod menu {
    use super::*;

    pub const MENU_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("MENU_NOT_FOUND", "menu not found");
    pub const MENU_INVALID_PAYLOAD: ErrorSpec =
        ErrorSpec::validation("MENU_INVALID_PAYLOAD", "invalid menu payload");
    pub const ROOT_AUTHORITY_IMMUTABLE: ErrorSpec = ErrorSpec::failed_precondition(
        "ROOT_AUTHORITY_IMMUTABLE",
        "default role permissions cannot be changed",
    );
    pub const MENU_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("MENU_DB_FAILED", "menu operation failed");
}

pub mod api_registry {
    use super::*;

    pub const API_EXISTS: ErrorSpec = ErrorSpec::conflict("API_EXISTS", "API already exists");
    pub const API_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("API_NOT_FOUND", "API not found");
    pub const API_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("API_DB_FAILED", "API operation failed");
}

pub mod users {
    use super::*;

    pub const INVALID_CREDENTIALS: ErrorSpec =
        ErrorSpec::unauthorized("INVALID_CREDENTIALS", "invalid username or password");
    pub const USER_DISABLED: ErrorSpec = ErrorSpec::forbidden("USER_DISABLED", "user is disabled");
    pub const USER_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("USER_NOT_FOUND", "user not found");
    pub const USER_ALREADY_EXISTS: ErrorSpec =
        ErrorSpec::conflict("USER_ALREADY_EXISTS", "user already exists");
    pub const INVALID_PASSWORD: ErrorSpec =
        ErrorSpec::bad_request("INVALID_PASSWORD", "invalid password");
    pub const USER_OPERATION_FAILED: ErrorSpec =
        ErrorSpec::internal("USER_OPERATION_FAILED", "user operation failed");
}

pub mod params {
    use super::*;

    pub const PARAM_JSON_ENCODE_FAILED: ErrorSpec =
        ErrorSpec::internal("PARAM_JSON_ENCODE_FAILED", "failed to serialize params");
}

pub mod depts {
    use super::*;

    pub const DEPT_INVALID_PARENT: ErrorSpec =
        ErrorSpec::validation("DEPT_INVALID_PARENT", "invalid department parent");
    pub const DEPT_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("DEPT_DB_FAILED", "department operation failed");
}

pub mod roles {
    use super::*;

    pub const ROLE_NOT_FOUND: ErrorSpec = ErrorSpec::not_found("ROLE_NOT_FOUND", "role not found");
    pub const ROLE_IMMUTABLE: ErrorSpec =
        ErrorSpec::failed_precondition("ROLE_IMMUTABLE", "system role cannot be deleted");
    pub const ROLE_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("ROLE_DB_FAILED", "role operation failed");
}

pub mod permissions {
    use super::*;

    pub const PERMISSION_NOT_FOUND: ErrorSpec =
        ErrorSpec::not_found("PERMISSION_NOT_FOUND", "permission not found");
    pub const PERMISSION_INVALID_CODE: ErrorSpec =
        ErrorSpec::validation("PERMISSION_INVALID_CODE", "invalid permission code");
    pub const PERMISSION_DB_FAILED: ErrorSpec =
        ErrorSpec::internal("PERMISSION_DB_FAILED", "permission operation failed");
}
