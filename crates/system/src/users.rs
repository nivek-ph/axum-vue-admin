use admin_httpz::AppError;
use auth::{
    jwt::JwtService,
    password::{AuthError, PasswordService},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::authority;
use crate::errors;

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
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "passWord")]
    pub password: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "headerImg")]
    pub header_img: Option<String>,
    #[serde(rename = "authorityId")]
    pub authority_id: Option<i64>,
    #[serde(rename = "authorityIds")]
    pub authority_ids: Option<Vec<i64>>,
    pub enable: Option<i32>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "headerImg")]
    pub header_img: String,
    pub enable: i32,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePasswordRequest {
    pub password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetSelfInfoRequest {
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    #[serde(rename = "headerImg")]
    pub header_img: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetSelfSettingRequest {
    #[serde(flatten)]
    pub origin_setting: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteUserRequest {
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResetPasswordRequest {
    #[serde(rename = "ID")]
    pub id: i64,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetUserAuthoritiesRequest {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "authorityIds")]
    pub authority_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetUserListRequest {
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub username: Option<String>,
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "orderKey")]
    pub order_key: Option<String>,
    pub desc: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserInfoView {
    #[serde(rename = "ID")]
    pub id: i64,
    pub uuid: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "headerImg")]
    pub header_img: String,
    pub authority: authority::AuthorityView,
    pub authorities: Vec<authority::AuthorityView>,
    pub enable: i32,
    pub phone: String,
    pub email: String,
    #[serde(rename = "originSetting")]
    pub origin_setting: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: i64,
    pub authority_id: i64,
    pub user: UserInfoView,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResult {
    pub user: UserInfoView,
    pub token: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("用户名或密码错误")]
    InvalidCredentials,
    #[error("用户已被禁用")]
    Disabled,
    #[error("用户不存在")]
    UserNotFound,
    #[error("用户已存在")]
    UserAlreadyExists,
    #[error("密码错误")]
    InvalidPassword,
    #[error("{0}")]
    Auth(#[from] AuthError),
    #[error("{0}")]
    Database(#[from] sqlx::Error),
}

impl From<LoginError> for AppError {
    fn from(error: LoginError) -> Self {
        match error {
            LoginError::InvalidCredentials => errors::users::INVALID_CREDENTIALS.into(),
            LoginError::Disabled => errors::users::USER_DISABLED.into(),
            LoginError::UserNotFound => errors::users::USER_NOT_FOUND.into(),
            LoginError::UserAlreadyExists => errors::users::USER_ALREADY_EXISTS.into(),
            LoginError::InvalidPassword => errors::users::INVALID_PASSWORD.into(),
            LoginError::Auth(error) => errors::users::USER_OPERATION_FAILED
                .into_error()
                .with_source(error),
            LoginError::Database(error) => errors::users::USER_OPERATION_FAILED
                .into_error()
                .with_source(error),
        }
    }
}

pub async fn ensure_admin_user(
    pool: &sqlx::PgPool,
    password_service: &PasswordService,
    username: &str,
    password: &str,
    nick_name: &str,
) -> Result<(), sqlx::Error> {
    if find_by_username(pool, username).await?.is_some() {
        return Ok(());
    }

    let password_hash = password_service
        .hash_password(password)
        .map_err(|err| sqlx::Error::Protocol(err.to_string()))?;

    sqlx::query(
        r#"
        insert into sys_users (
            uuid,
            username,
            password_hash,
            nick_name,
            header_img,
            authority_id,
            authority_name,
            default_router,
            enable,
            phone,
            email,
            origin_setting
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, true, null, null, null)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(username)
    .bind(password_hash)
    .bind(nick_name)
    .bind("https://qmplusimg.henrongyi.top/gva_header.jpg")
    .bind(888_i64)
    .bind("超级管理员")
    .bind("dashboard")
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn register_user(
    pool: &sqlx::PgPool,
    password_service: &PasswordService,
    payload: RegisterRequest,
) -> Result<(), LoginError> {
    if find_by_username(pool, &payload.user_name).await?.is_some() {
        return Err(LoginError::UserAlreadyExists);
    }
    let authority = authority_from_ids(payload.authority_id, payload.authority_ids.as_ref());
    let password_hash = password_service.hash_password(&payload.password)?;

    sqlx::query(
        r#"
        insert into sys_users (
            uuid,
            username,
            password_hash,
            nick_name,
            header_img,
            authority_id,
            authority_name,
            default_router,
            enable,
            phone,
            email,
            origin_setting
        ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, null)
        "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&payload.user_name)
    .bind(password_hash)
    .bind(&payload.nick_name)
    .bind(
        payload
            .header_img
            .unwrap_or_else(|| "https://qmplusimg.henrongyi.top/gva_header.jpg".to_string()),
    )
    .bind(authority.authority_id)
    .bind(&authority.authority_name)
    .bind(&authority.default_router)
    .bind(payload.enable.unwrap_or(1) == 1)
    .bind(payload.phone)
    .bind(payload.email)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn login(
    pool: &sqlx::PgPool,
    password_service: &PasswordService,
    jwt_service: &JwtService,
    payload: LoginRequest,
) -> Result<LoginResult, LoginError> {
    let record = find_by_username(pool, &payload.username)
        .await?
        .ok_or(LoginError::InvalidCredentials)?;

    if !record.enable {
        return Err(LoginError::Disabled);
    }

    let verified = password_service.verify_password(&payload.password, &record.password_hash)?;
    if !verified {
        return Err(LoginError::InvalidCredentials);
    }

    let token = jwt_service.issue_token(record.id, &record.username, record.authority_id)?;

    Ok(LoginResult {
        user: build_user_info(&record),
        token,
    })
}

pub async fn load_authenticated_user(
    pool: &sqlx::PgPool,
    user_id: i64,
) -> Result<AuthenticatedUser, LoginError> {
    let record = find_by_id(pool, user_id)
        .await?
        .ok_or(LoginError::UserNotFound)?;

    Ok(AuthenticatedUser {
        id: record.id,
        authority_id: record.authority_id,
        user: build_user_info(&record),
    })
}

pub async fn get_user_list(
    pool: &sqlx::PgPool,
    query: GetUserListRequest,
) -> Result<(Vec<UserInfoView>, i64), LoginError> {
    let page = query.page.max(1);
    let page_size = query.page_size.max(1);
    let offset = (page - 1) * page_size;
    let order_key = match query.order_key.as_deref() {
        Some("username") => "username",
        Some("nick_name") => "nick_name",
        Some("phone") => "phone",
        Some("email") => "email",
        _ => "id",
    };
    let order_dir = if query.desc.unwrap_or(true) {
        "desc"
    } else {
        "asc"
    };
    let order_clause = format!("{order_key} {order_dir}");

    let total: i64 = sqlx::query_scalar(
        r#"
        select count(*) from sys_users
        where ($1::text is null or username ilike '%' || $1 || '%')
          and ($2::text is null or nick_name ilike '%' || $2 || '%')
          and ($3::text is null or coalesce(phone, '') ilike '%' || $3 || '%')
          and ($4::text is null or coalesce(email, '') ilike '%' || $4 || '%')
        "#,
    )
    .bind(query.username.as_deref())
    .bind(query.nick_name.as_deref())
    .bind(query.phone.as_deref())
    .bind(query.email.as_deref())
    .fetch_one(pool)
    .await?;

    let sql = format!(
        r#"
        select
            id,
            uuid,
            username,
            password_hash,
            nick_name,
            header_img,
            authority_id,
            authority_name,
            default_router,
            enable,
            phone,
            email,
            origin_setting
        from sys_users
        where ($1::text is null or username ilike '%' || $1 || '%')
          and ($2::text is null or nick_name ilike '%' || $2 || '%')
          and ($3::text is null or coalesce(phone, '') ilike '%' || $3 || '%')
          and ($4::text is null or coalesce(email, '') ilike '%' || $4 || '%')
        order by {order_clause}
        limit $5 offset $6
        "#
    );

    let rows = sqlx::query_as::<_, UserRecord>(sqlx::AssertSqlSafe(sql))
        .bind(query.username.as_deref())
        .bind(query.nick_name.as_deref())
        .bind(query.phone.as_deref())
        .bind(query.email.as_deref())
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok((rows.iter().map(build_user_info).collect(), total))
}

pub async fn update_user(
    pool: &sqlx::PgPool,
    payload: UpdateUserRequest,
) -> Result<(), LoginError> {
    sqlx::query(
        r#"
        update sys_users
        set nick_name = $1,
            header_img = $2,
            enable = $3,
            phone = $4,
            email = $5,
            updated_at = now()
        where id = $6
        "#,
    )
    .bind(payload.nick_name)
    .bind(payload.header_img)
    .bind(payload.enable == 1)
    .bind(payload.phone)
    .bind(payload.email)
    .bind(payload.id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_user(
    pool: &sqlx::PgPool,
    payload: DeleteUserRequest,
) -> Result<(), LoginError> {
    sqlx::query("delete from sys_users where id = $1")
        .bind(payload.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn reset_password(
    pool: &sqlx::PgPool,
    password_service: &PasswordService,
    payload: ResetPasswordRequest,
) -> Result<(), LoginError> {
    let password_hash = password_service.hash_password(&payload.password)?;
    sqlx::query(
        r#"
        update sys_users
        set password_hash = $1,
            updated_at = now()
        where id = $2
        "#,
    )
    .bind(password_hash)
    .bind(payload.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_user_authorities(
    pool: &sqlx::PgPool,
    payload: SetUserAuthoritiesRequest,
) -> Result<(), LoginError> {
    let authority = authority_from_ids(None, Some(&payload.authority_ids));
    sqlx::query(
        r#"
        update sys_users
        set authority_id = $1,
            authority_name = $2,
            default_router = $3,
            updated_at = now()
        where id = $4
        "#,
    )
    .bind(authority.authority_id)
    .bind(authority.authority_name)
    .bind(authority.default_router)
    .bind(payload.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_user_authority(
    pool: &sqlx::PgPool,
    user_id: i64,
    authority_id: i64,
) -> Result<(), LoginError> {
    let authority = authority_from_ids(Some(authority_id), None);
    sqlx::query(
        r#"
        update sys_users
        set authority_id = $1,
            authority_name = $2,
            default_router = $3,
            updated_at = now()
        where id = $4
        "#,
    )
    .bind(authority.authority_id)
    .bind(authority.authority_name)
    .bind(authority.default_router)
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn change_password(
    pool: &sqlx::PgPool,
    password_service: &PasswordService,
    user_id: i64,
    payload: ChangePasswordRequest,
) -> Result<(), LoginError> {
    let record = find_by_id(pool, user_id)
        .await?
        .ok_or(LoginError::UserNotFound)?;
    let verified = password_service.verify_password(&payload.password, &record.password_hash)?;
    if !verified {
        return Err(LoginError::InvalidPassword);
    }

    let password_hash = password_service.hash_password(&payload.new_password)?;
    sqlx::query(
        r#"
        update sys_users
        set password_hash = $1,
            updated_at = now()
        where id = $2
        "#,
    )
    .bind(password_hash)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_self_info(
    pool: &sqlx::PgPool,
    user_id: i64,
    payload: SetSelfInfoRequest,
) -> Result<(), LoginError> {
    sqlx::query(
        r#"
        update sys_users
        set nick_name = coalesce($1, nick_name),
            header_img = coalesce($2, header_img),
            phone = coalesce($3, phone),
            email = coalesce($4, email),
            updated_at = now()
        where id = $5
        "#,
    )
    .bind(payload.nick_name)
    .bind(payload.header_img)
    .bind(payload.phone)
    .bind(payload.email)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_self_setting(
    pool: &sqlx::PgPool,
    user_id: i64,
    payload: SetSelfSettingRequest,
) -> Result<(), LoginError> {
    sqlx::query(
        r#"
        update sys_users
        set origin_setting = $1,
            updated_at = now()
        where id = $2
        "#,
    )
    .bind(payload.origin_setting)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn find_by_username(
    pool: &sqlx::PgPool,
    username: &str,
) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        select
            id,
            uuid,
            username,
            password_hash,
            nick_name,
            header_img,
            authority_id,
            authority_name,
            default_router,
            enable,
            phone,
            email,
            origin_setting
        from sys_users
        where username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

async fn find_by_id(pool: &sqlx::PgPool, user_id: i64) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>(
        r#"
        select
            id,
            uuid,
            username,
            password_hash,
            nick_name,
            header_img,
            authority_id,
            authority_name,
            default_router,
            enable,
            phone,
            email,
            origin_setting
        from sys_users
        where id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

fn build_user_info(record: &UserRecord) -> UserInfoView {
    let authority = authority::AuthorityView {
        authority_id: record.authority_id,
        authority_name: record.authority_name.clone(),
        parent_id: 0,
        default_router: record.default_router.clone(),
        children: Vec::new(),
        data_authority_id: Vec::new(),
    };

    UserInfoView {
        id: record.id,
        uuid: record.uuid.clone(),
        user_name: record.username.clone(),
        nick_name: record.nick_name.clone(),
        header_img: record.header_img.clone(),
        authority: authority.clone(),
        authorities: vec![authority],
        enable: if record.enable { 1 } else { 2 },
        phone: record.phone.clone().unwrap_or_default(),
        email: record.email.clone().unwrap_or_default(),
        origin_setting: record.origin_setting.clone(),
    }
}

fn authority_from_ids(
    authority_id: Option<i64>,
    authority_ids: Option<&Vec<i64>>,
) -> authority::AuthorityView {
    let requested_id = authority_ids
        .and_then(|ids| ids.first().copied())
        .or(authority_id)
        .unwrap_or(888);

    authority::default_authorities()
        .into_iter()
        .find(|item| item.authority_id == requested_id)
        .unwrap_or_else(|| authority::default_authorities()[0].clone())
}
