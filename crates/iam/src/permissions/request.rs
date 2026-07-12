use super::PermissionApiBinding;

#[derive(Debug, Clone)]
pub struct PermissionPayload {
    pub module_key: String,
    pub resource: String,
    pub action: String,
    pub code: String,
    pub name: String,
    pub permission_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PermissionApisPayload {
    pub apis: Vec<PermissionApiBinding>,
}
