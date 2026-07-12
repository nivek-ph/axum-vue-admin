use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, put},
};
use serde_json::Value;

use super::dto::{
    PermissionApiResponse, PermissionApisPayload, PermissionPayload, PermissionResponse,
};
use super::error::map_error;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_permissions).post(create_permission))
        .route("/{id}", put(update_permission).delete(delete_permission))
        .route(
            "/{id}/apis",
            get(get_api_permissions).put(set_api_permissions),
        )
}

pub async fn get_permissions(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .permissions
        .list()
        .await
        .map_err(map_error)?
        .into_iter()
        .map(PermissionResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn create_permission(
    State(state): State<AppState>,
    Json(payload): Json<PermissionPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let permission = PermissionResponse::from(
        state
            .permissions
            .create(payload.into())
            .await
            .map_err(map_error)?,
    );

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "permission": permission,
    }))))
}

pub async fn update_permission(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<PermissionPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let permission = PermissionResponse::from(
        state
            .permissions
            .update(id, payload.into())
            .await
            .map_err(map_error)?,
    );

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "permission": permission,
    }))))
}

pub async fn delete_permission(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.permissions.delete(id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn get_api_permissions(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let apis = state
        .permissions
        .api_bindings(id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(PermissionApiResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({ "apis": apis }))))
}

pub async fn set_api_permissions(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<PermissionApisPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .permissions
        .set_api_bindings(
            id,
            payload
                .apis
                .into_iter()
                .map(|api| iam::permissions::PermissionApiBinding {
                    method: api.method,
                    path_pattern: api.path_pattern,
                })
                .collect(),
        )
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("saved")))
}
