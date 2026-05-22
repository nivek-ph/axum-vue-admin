use admin_httpz::{ApiResponse, AppError};
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::Value;

use crate::state::AppState;

pub async fn get_category_list(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let list = file_storage::category::list(&state.pool).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!(list))))
}

pub async fn add_category(
    State(state): State<AppState>,
    Json(payload): Json<file_storage::category::CategoryPayload>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    file_storage::category::upsert(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("操作成功")))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Json(payload): Json<file_storage::category::DeleteCategoryPayload>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    file_storage::category::delete(&state.pool, payload.id).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_category_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    file_storage::category::delete(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}
