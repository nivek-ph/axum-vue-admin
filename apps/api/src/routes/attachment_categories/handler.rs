use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get},
};
use serde_json::Value;

use super::dto::{CategoryPayload, CategoryResponse, DeleteCategoryPayload};
use super::error::map_error;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_category_list).post(add_category))
        .route("/{id}", delete(delete_category_by_id))
}

pub async fn get_category_list(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .attachment_categories
        .list()
        .await
        .map_err(map_error)?
        .into_iter()
        .map(CategoryResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!(list))))
}

pub async fn add_category(
    State(state): State<AppState>,
    Json(payload): Json<CategoryPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .attachment_categories
        .upsert(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("operation succeeded")))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Json(payload): Json<DeleteCategoryPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .attachment_categories
        .delete(payload.id)
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_category_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .attachment_categories
        .delete(id)
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}
