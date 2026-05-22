use admin_httpz::ApiResponse;
use axum::Json;
use serde_json::Value;

pub async fn check_db() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "needInit": false
    })))
}

pub async fn init_db() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("自动创建数据库成功"))
}
