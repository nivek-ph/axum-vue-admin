use sqlx::FromRow;
#[derive(Debug, Clone)]
pub struct OperationUserView {
    pub user_name: String,
    pub nick_name: String,
}
#[derive(Debug, Clone, FromRow)]
pub(super) struct OperationLogRow {
    pub id: i64,
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status: i32,
    pub agent: String,
    pub error_message: String,
    pub body: String,
    pub resp: String,
    pub created_at: String,
    pub user_name: String,
    pub nick_name: String,
}
#[derive(Debug, Clone)]
pub struct OperationLogView {
    pub id: i64,
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status: i32,
    pub agent: String,
    pub error_message: String,
    pub body: String,
    pub resp: String,
    pub created_at: String,
    pub user: OperationUserView,
}
