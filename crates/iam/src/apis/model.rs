use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ApiRecord {
    pub id: i64,
    pub path: String,
    pub description: String,
    pub api_group: String,
    pub method: String,
}

#[derive(Debug, Clone)]
pub struct ApiRoleSelection {
    pub authority_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiRoleMatrixItem {
    pub path: String,
    pub method: String,
    pub authority_ids: Vec<i64>,
}
