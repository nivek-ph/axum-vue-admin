#[derive(Debug, Clone)]
pub struct SearchApiRequest {
    pub page: i64,
    pub page_size: i64,
    pub path: Option<String>,
    pub description: Option<String>,
    pub api_group: Option<String>,
    pub method: Option<String>,
    pub order_key: Option<String>,
    pub desc: Option<bool>,
}
#[derive(Debug, Clone)]
pub struct ApiIdRequest {
    pub id: i64,
}
#[derive(Debug, Clone)]
pub struct DeleteApisByIdsRequest {
    pub ids: Vec<i64>,
}
#[derive(Debug, Clone)]
pub struct ApiPayload {
    pub id: i64,
    pub path: String,
    pub description: String,
    pub api_group: String,
    pub method: String,
}
#[derive(Debug, Clone)]
pub struct ApiRoleQuery {
    pub path: String,
    pub method: String,
}
#[derive(Debug, Clone)]
pub struct AuthorityApiQuery {
    pub authority_id: i64,
}
#[derive(Debug, Clone)]
pub struct SetApiRolesRequest {
    pub path: String,
    pub method: String,
    pub authority_ids: Vec<i64>,
}
