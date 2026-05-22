use admin_httpz::ApiResponse;
use axum::Json;
use serde_json::Value;

#[utoipa::path(
    post,
    path = "/api/auth/captcha",
    tag = "auth",
    responses(
        (status = 200, description = "Captcha config", body = crate::docs::CaptchaResponse)
    )
)]
pub async fn captcha() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "captchaLength": 0,
        "picPath": "",
        "captchaId": "",
        "openCaptcha": false
    })))
}
