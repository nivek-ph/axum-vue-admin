pub const SUPER_ADMIN_AUTHORITY_ID: i64 = 1;

#[derive(Debug, Clone)]
pub struct AuthorityView {
    pub authority_id: i64,
    pub authority_name: String,
    pub parent_id: i64,
    pub default_router: String,
    pub children: Vec<AuthorityView>,
    pub data_authority_id: Vec<AuthorityDataView>,
}

#[derive(Debug, Clone)]
pub struct AuthorityDataView {
    pub authority_id: i64,
    pub authority_name: String,
}

pub fn default_authorities() -> Vec<AuthorityView> {
    vec![AuthorityView {
        authority_id: SUPER_ADMIN_AUTHORITY_ID,
        authority_name: "Super Admin".to_string(),
        parent_id: 0,
        default_router: "dashboard".to_string(),
        children: Vec::new(),
        data_authority_id: Vec::new(),
    }]
}
