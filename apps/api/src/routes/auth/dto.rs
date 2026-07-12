use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub captcha: String,
    #[serde(rename = "captchaId")]
    pub captcha_id: String,
}
