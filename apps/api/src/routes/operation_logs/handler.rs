use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get},
};
use serde_json::Value;

use super::dto::{IdRequest, IdsRequest, OperationLogResponse, OperationLogSearch};
use super::error::map_error;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_operation_log_list).delete(delete_operation_log_by_ids),
        )
        .route("/{id}", delete(delete_operation_log_by_id))
}

pub async fn get_operation_log_list(
    State(state): State<AppState>,
    Query(payload): Query<OperationLogSearch>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = state
        .operation_logs
        .list(payload.into())
        .await
        .map_err(map_error)?;
    let list = list
        .into_iter()
        .map(OperationLogResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn delete_operation_log(
    State(state): State<AppState>,
    Json(payload): Json<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .operation_logs
        .delete(payload.id)
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_operation_log_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.operation_logs.delete(id).await.map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_operation_log_by_ids(
    State(state): State<AppState>,
    Json(payload): Json<IdsRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .operation_logs
        .delete_many(payload.ids)
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("batch deleted")))
}
