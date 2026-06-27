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
        .route("/", get(get_roles).post(create_role))
        .route("/{id}", put(update_role).delete(delete_role))
        .route(
            "/{id}/permissions",
            get(get_role_permissions).put(set_role_permissions),
        )
        .route("/{id}/depts", get(get_role_depts).put(set_role_depts))
        .route("/{id}/users", get(get_role_users).put(set_role_users))
}

pub async fn get_roles(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let list = system::roles::list(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn create_role(
    State(state): State<AppState>,
    Json(payload): Json<system::roles::RolePayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let role = system::roles::create(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "role": role }))))
}

pub async fn update_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<system::roles::RolePayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let role = system::roles::update(&state.pool, id, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({ "role": role }))))
}

pub async fn delete_role(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::roles::delete(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn get_role_permissions(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let permission_ids = system::roles::permission_ids(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "permissionIds": permission_ids,
    }))))
}

pub async fn set_role_permissions(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<system::roles::RolePermissionPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::roles::set_permission_ids(&state.pool, id, payload.permission_ids).await?;

    Ok(Json(ApiResponse::ok_message("saved")))
}

pub async fn get_role_depts(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let dept_ids = system::roles::dept_ids(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "deptIds": dept_ids,
    }))))
}

pub async fn set_role_depts(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<system::roles::RoleDeptPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::roles::set_dept_ids(&state.pool, id, payload.dept_ids).await?;

    Ok(Json(ApiResponse::ok_message("saved")))
}

pub async fn get_role_users(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let user_ids = system::roles::user_ids(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(user_ids))))
}

pub async fn set_role_users(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<system::roles::RoleUsersPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::roles::set_user_ids(&state.pool, id, payload.user_ids).await?;

    Ok(Json(ApiResponse::ok_message("saved")))
}
