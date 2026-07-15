use crate::{ApiResponse, AppResult};
use axum::{Json, extract::State};
use serde_json::Value;

use crate::state::AppState;

#[utoipa::path(
    post,
    path = "/api/auth/captcha",
    tag = "auth",
    responses(
        (status = 200, description = "Captcha config", body = crate::docs::CaptchaResponse)
    )
)]
pub async fn captcha(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let challenge = state.captcha.create().await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "captchaLength": 4,
        "picPath": challenge.image,
        "captchaId": challenge.id,
        "openCaptcha": true
    }))))
}
