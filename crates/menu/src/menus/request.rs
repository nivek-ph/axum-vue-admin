#[derive(Debug, Clone)]
pub struct SetMenuRolesRequest {
    pub menu_id: i64,
    pub authority_ids: Vec<i64>,
}
