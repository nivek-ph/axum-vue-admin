use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, put},
};
use serde_json::Value;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_permissions).post(create_permission))
        .route("/{id}", put(update_permission).delete(delete_permission))
        .route(
            "/{id}/apis",
            get(get_permission_apis).put(set_permission_apis),
        )
}

pub async fn get_permissions(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let list = system::permissions::list(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn create_permission(
    State(state): State<AppState>,
    Json(payload): Json<system::permissions::PermissionPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let permission = system::permissions::create(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "permission": permission,
    }))))
}

pub async fn update_permission(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<system::permissions::PermissionPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let permission = system::permissions::update(&state.pool, id, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "permission": permission,
    }))))
}

pub async fn delete_permission(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::permissions::delete(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn get_permission_apis(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let apis = system::permissions::api_bindings(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "apis": apis }))))
}

pub async fn set_permission_apis(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<system::permissions::PermissionApisPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::permissions::set_api_bindings(&state.pool, id, payload.apis).await?;

    Ok(Json(ApiResponse::ok_message("saved")))
}
