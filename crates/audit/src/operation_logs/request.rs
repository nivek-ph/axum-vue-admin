#[derive(Debug, Clone)]
pub struct CreateOperationLog {
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status: i32,
    pub agent: String,
    pub error_message: String,
    pub body: String,
    pub resp: String,
    pub user_id: i64,
}
#[derive(Debug, Clone)]
pub struct OperationLogSearch {
    pub page: i64,
    pub page_size: i64,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i32>,
}
