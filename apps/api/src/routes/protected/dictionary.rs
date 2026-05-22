use admin_httpz::{ApiResponse, AppError, OptionAppExt};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;

use crate::request::errors;
use crate::state::AppState;

pub async fn create_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<system::dictionary::SysDictionary>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::dictionary::create(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("创建成功")))
}

pub async fn update_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<system::dictionary::SysDictionary>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::dictionary::update(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("更新成功")))
}

pub async fn update_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<system::dictionary::SysDictionary>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    payload.id = id;
    system::dictionary::update(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("更新成功")))
}

pub async fn find_sys_dictionary(
    State(state): State<AppState>,
    Query(payload): Query<system::dictionary::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let item =
        system::dictionary::find_by_query(&state.pool, payload.id, payload.dict_type).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "resysDictionary": item.unwrap_or_else(|| serde_json::json!({}))
    }))))
}

pub async fn find_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let item = system::dictionary::find_by_query(&state.pool, Some(id), None).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "resysDictionary": item.unwrap_or_else(|| serde_json::json!({}))
    }))))
}

pub async fn get_sys_dictionary_list(
    State(state): State<AppState>,
    Query(payload): Query<system::dictionary::DictionaryListQuery>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let list = system::dictionary::list(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok(serde_json::json!(list))))
}

pub async fn delete_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<system::dictionary::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    system::dictionary::delete(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::dictionary::delete(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn export_sys_dictionary(
    State(state): State<AppState>,
    Query(payload): Query<system::dictionary::IdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    let data = system::dictionary::export_dictionary(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok(
        data.unwrap_or_else(|| serde_json::json!({})),
    )))
}

pub async fn export_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let data = system::dictionary::export_dictionary(&state.pool, id).await?;
    Ok(Json(ApiResponse::ok(
        data.unwrap_or_else(|| serde_json::json!({})),
    )))
}

pub async fn import_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<system::dictionary::ImportDictionaryPayload>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::dictionary::import_dictionary(&state.pool, payload).await?;
    Ok(Json(ApiResponse::ok_message("导入成功")))
}
