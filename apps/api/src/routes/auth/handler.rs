use admin_httpz::{ApiResponse, AppResult};
use axum::{Json, extract::State, http::HeaderMap};
use serde_json::Value;

use super::LoginRequest;
use super::error::map_error;
use super::operation::LoginInput;
use crate::routes::users::dto::UserResponse;
use crate::state::AppState;

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login success", body = crate::docs::LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let result = state
        .login
        .execute(LoginInput {
            username: payload.username,
            password: payload.password,
            captcha: payload.captcha,
            captcha_id: payload.captcha_id,
            ip: header_value(&headers, "x-forwarded-for"),
            agent: header_value(&headers, "user-agent"),
        })
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "user": UserResponse::from(result.user),
        "token": result.token,
    }))))
}

fn header_value(headers: &HeaderMap, key: &str) -> String {
    headers
        .get(key)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string()
}
