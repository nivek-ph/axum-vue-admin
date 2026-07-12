use sqlx::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct LoginLogView {
    pub id: i64,
    pub username: String,
    pub ip: String,
    pub status: bool,
    pub error_message: String,
    pub agent: String,
    pub created_at: String,
}
