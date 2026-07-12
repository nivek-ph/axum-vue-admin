use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct PermissionSummary {
    pub id: i64,
    pub module_key: String,
    pub resource: String,
    pub action: String,
    pub code: String,
    pub name: String,
    pub permission_type: String,
    pub status: String,
}

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct PermissionApiBinding {
    pub method: String,
    pub path_pattern: String,
}
