use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: i64,
    pub module_key: String,
    pub resource: String,
    pub action: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub permission_type: String,
    pub status: String,
}
impl From<iam::permissions::PermissionSummary> for PermissionResponse {
    fn from(v: iam::permissions::PermissionSummary) -> Self {
        Self {
            id: v.id,
            module_key: v.module_key,
            resource: v.resource,
            action: v.action,
            code: v.code,
            name: v.name,
            permission_type: v.permission_type,
            status: v.status,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PermissionApiResponse {
    pub method: String,
    pub path_pattern: String,
}
impl From<iam::permissions::PermissionApiBinding> for PermissionApiResponse {
    fn from(v: iam::permissions::PermissionApiBinding) -> Self {
        Self {
            method: v.method,
            path_pattern: v.path_pattern,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PermissionPayload {
    #[serde(alias = "moduleKey")]
    pub module_key: String,
    pub resource: String,
    pub action: String,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    pub permission_type: Option<String>,
    pub status: Option<String>,
}
impl From<PermissionPayload> for iam::permissions::PermissionPayload {
    fn from(v: PermissionPayload) -> Self {
        Self {
            module_key: v.module_key,
            resource: v.resource,
            action: v.action,
            code: v.code,
            name: v.name,
            permission_type: v.permission_type,
            status: v.status,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PermissionApiBinding {
    pub method: String,
    #[serde(alias = "pathPattern")]
    pub path_pattern: String,
}
#[derive(Debug, Deserialize)]
pub struct PermissionApisPayload {
    pub apis: Vec<PermissionApiBinding>,
}
