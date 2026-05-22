use admin_httpz::ApiResponse;
use axum::Json;
use serde_json::Value;

pub async fn json_in_blacklist() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("拉黑成功"))
}
