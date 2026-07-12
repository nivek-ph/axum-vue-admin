use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Dept {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeptNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub status: String,
    pub children: Vec<DeptNode>,
}
