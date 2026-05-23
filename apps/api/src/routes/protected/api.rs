use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;

use crate::state::AppState;

pub async fn get_api_list(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::SearchApiRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = system::api_registry::get_api_list(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn get_api_list_by_query(
    State(state): State<AppState>,
    Query(payload): Query<system::api_registry::SearchApiRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = system::api_registry::get_api_list(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn create_api(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::ApiPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::api_registry::create_api(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("创建成功")))
}

pub async fn get_api_by_id(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::ApiIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let api = system::api_registry::get_api_by_id(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "api": api,
    }))))
}

pub async fn get_api_by_path_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let api = system::api_registry::get_api_by_id(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "api": api,
    }))))
}

pub async fn update_api(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::ApiPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::api_registry::update_api(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("修改成功")))
}

pub async fn update_api_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<system::api_registry::ApiPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.id = id;
    system::api_registry::update_api(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("修改成功")))
}

pub async fn delete_api(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::ApiIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::api_registry::delete_api(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_api_by_path_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::api_registry::delete_api(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_apis_by_ids(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::DeleteApisByIdsRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::api_registry::delete_apis_by_ids(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn fresh_casbin() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("刷新成功"))
}

pub async fn sync_api() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "newApis": [],
        "deleteApis": [],
        "ignoreApis": [],
    })))
}

pub async fn get_api_groups(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let (groups, api_group_map) = system::api_registry::get_api_groups(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "groups": groups,
        "apiGroupMap": api_group_map,
    }))))
}

pub async fn ignore_api() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("忽略成功"))
}

pub async fn enter_sync_api() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("同步成功"))
}

pub async fn get_all_apis(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let apis = system::api_registry::get_all_apis(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "apis": apis }))))
}

pub async fn get_api_roles(
    State(state): State<AppState>,
    Query(payload): Query<system::api_registry::ApiRoleQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let selection = system::api_registry::get_api_roles(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(selection))))
}

pub async fn set_api_roles(
    State(state): State<AppState>,
    Json(payload): Json<system::api_registry::SetApiRolesRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::api_registry::set_api_roles(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("分配成功")))
}
