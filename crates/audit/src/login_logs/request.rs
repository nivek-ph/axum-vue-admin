#[derive(Debug, Clone)]
pub struct CreateLoginLog {
    pub username: String,
    pub ip: String,
    pub status: bool,
    pub error_message: String,
    pub agent: String,
    pub user_id: Option<i64>,
}
#[derive(Debug, Clone)]
pub struct LoginLogSearch {
    pub page: i64,
    pub page_size: i64,
    pub username: Option<String>,
    pub status: Option<bool>,
}
