use admin_httpz::{ApiResponse, AppError};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;

use crate::state::AppState;

pub async fn get_login_log_list(
    State(state): State<AppState>,
    Query(payload): Query<system::logs::LoginLogSearch>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = system::logs::get_login_log_list(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn find_login_log(
    State(state): State<AppState>,
    Query(payload): Query<system::logs::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let (list, _) = system::logs::get_login_log_list(
        &state.pool,
        system::logs::LoginLogSearch {
            page: 1,
            page_size: 9999,
            username: None,
            status: None,
        },
    )
    .await?;
    let item = list.into_iter().find(|log| log.id == payload.id);
    Ok(Json(ApiResponse::ok(match item {
        Some(log) => serde_json::json!(log),
        None => serde_json::json!({}),
    })))
}

pub async fn find_login_log_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let (list, _) = system::logs::get_login_log_list(
        &state.pool,
        system::logs::LoginLogSearch {
            page: 1,
            page_size: 9999,
            username: None,
            status: None,
        },
    )
    .await?;
    let item = list.into_iter().find(|log| log.id == id);
    Ok(Json(ApiResponse::ok(match item {
        Some(log) => serde_json::json!(log),
        None => serde_json::json!({}),
    })))
}

pub async fn delete_login_log(
    State(state): State<AppState>,
    Json(payload): Json<system::logs::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::logs::delete_login_log(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_login_log_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::logs::delete_login_log(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_login_log_by_ids(
    State(state): State<AppState>,
    Json(payload): Json<system::logs::IdsRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::logs::delete_login_logs(&state.pool, payload.ids).await?;

    Ok(Json(ApiResponse::ok_message("批量删除成功")))
}

pub async fn get_operation_log_list(
    State(state): State<AppState>,
    Query(payload): Query<system::logs::OperationLogSearch>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = system::logs::get_operation_log_list(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn delete_operation_log(
    State(state): State<AppState>,
    Json(payload): Json<system::logs::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::logs::delete_operation_log(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_operation_log_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::logs::delete_operation_log(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_operation_log_by_ids(
    State(state): State<AppState>,
    Json(payload): Json<system::logs::IdsRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::logs::delete_operation_logs(&state.pool, payload.ids).await?;

    Ok(Json(ApiResponse::ok_message("批量删除成功")))
}
