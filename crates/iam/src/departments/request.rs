#[derive(Debug, Clone)]
pub struct CreateDeptPayload {
    pub parent_id: Option<i64>,
    pub name: String,
    pub code: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateDeptPayload {
    pub parent_id: Option<i64>,
    pub name: String,
    pub code: String,
    pub sort: Option<i32>,
    pub status: Option<String>,
}
