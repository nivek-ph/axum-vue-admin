use sqlx::FromRow;

use crate::{authority, roles::RoleSummary};

#[derive(Debug, Clone, FromRow)]
pub struct UserRecord {
    pub id: i64,
    pub uuid: String,
    pub username: String,
    pub password_hash: String,
    pub nick_name: String,
    pub header_img: String,
    pub authority_id: i64,
    pub authority_name: String,
    pub default_router: String,
    pub enable: bool,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub origin_setting: Option<serde_json::Value>,
    pub dept_id: Option<i64>,
    pub dept_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserInfoView {
    pub id: i64,
    pub uuid: String,
    pub user_name: String,
    pub nick_name: String,
    pub header_img: String,
    pub authority: authority::AuthorityView,
    pub authorities: Vec<authority::AuthorityView>,
    pub enable: i32,
    pub phone: String,
    pub email: String,
    pub origin_setting: Option<serde_json::Value>,
    pub dept_id: Option<i64>,
    pub dept_name: String,
    pub roles: Vec<RoleSummary>,
    pub role_ids: Vec<i64>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: i64,
    pub authority_id: i64,
    pub user: UserInfoView,
}

#[derive(Debug, Clone)]
pub struct LoginIdentity {
    pub id: i64,
    pub username: String,
    pub authority_id: i64,
    pub user: UserInfoView,
}
