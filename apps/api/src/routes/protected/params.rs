use admin_httpz::{ApiResponse, AppError};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;
use system::errors;

use crate::state::AppState;

pub async fn create_sys_params(
    State(state): State<AppState>,
    Json(payload): Json<system::params::SysParam>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::params::create(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("创建成功")))
}

pub async fn update_sys_params(
    State(state): State<AppState>,
    Json(payload): Json<system::params::SysParam>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::params::update(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("更新成功")))
}

pub async fn update_sys_params_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<system::params::SysParam>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    payload.id = id;
    system::params::update(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("更新成功")))
}

pub async fn find_sys_params(
    State(state): State<AppState>,
    Query(payload): Query<system::params::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let item = system::params::find(&state.pool, payload.id).await?;
    Ok(Json(ApiResponse::ok(
        item.map(serde_json::to_value)
            .transpose()
            .map_err(|e| {
                errors::params::PARAM_JSON_ENCODE_FAILED
                    .into_error()
                    .with_source(e)
            })?
            .unwrap_or_else(|| serde_json::json!({})),
    )))
}

pub async fn find_sys_params_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let item = system::params::find(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok(
        item.map(serde_json::to_value)
            .transpose()
            .map_err(|e| {
                errors::params::PARAM_JSON_ENCODE_FAILED
                    .into_error()
                    .with_source(e)
            })?
            .unwrap_or_else(|| serde_json::json!({})),
    )))
}

pub async fn get_sys_params_list(
    State(state): State<AppState>,
    Query(payload): Query<system::params::ParamListQuery>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let (list, total, page, page_size) = system::params::list(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size
    }))))
}

pub async fn delete_sys_params(
    State(state): State<AppState>,
    Query(payload): Query<system::params::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::params::delete(&state.pool, payload.id).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_sys_params_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::params::delete(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_sys_params_by_ids(
    State(state): State<AppState>,
    Query(payload): Query<system::params::IdsRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::params::delete_many(&state.pool, payload.ids).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn get_sys_param(
    State(state): State<AppState>,
    Query(payload): Query<system::params::ParamListQuery>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let key = payload.key.unwrap_or_default();
    let item = system::params::get_by_key(&state.pool, &key).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "sysParam": item
    }))))
}
