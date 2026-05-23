use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;

use crate::state::AppState;

pub async fn get_authority_list(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let data = system::authority::get_authority_info_list(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(data))))
}

pub async fn create_authority(
    State(state): State<AppState>,
    Json(payload): Json<system::authority::CreateAuthorityRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let authority = system::authority::create_authority(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "authority": authority,
    }))))
}

pub async fn update_authority(
    State(state): State<AppState>,
    Json(payload): Json<system::authority::UpdateAuthorityRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let authority = system::authority::update_authority(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "authority": authority,
    }))))
}

pub async fn delete_authority(
    State(state): State<AppState>,
    Json(payload): Json<system::authority::DeleteAuthorityRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::authority::delete_authority(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_authority_by_id(
    State(state): State<AppState>,
    Path(authority_id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::authority::delete_authority(
        &state.pool,
        system::authority::DeleteAuthorityRequest { authority_id },
    )
    .await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn copy_authority(
    State(state): State<AppState>,
    Json(payload): Json<system::authority::CopyAuthorityRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let authority = system::authority::copy_authority(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "authority": authority,
    }))))
}

pub async fn get_users_by_authority(
    State(state): State<AppState>,
    Query(payload): Query<system::menu::MenuAuthorityRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let user_ids =
        system::authority::get_user_ids_by_authority_id(&state.pool, payload.authority_id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(user_ids))))
}

pub async fn get_users_by_authority_id(
    State(state): State<AppState>,
    Path(authority_id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let user_ids =
        system::authority::get_user_ids_by_authority_id(&state.pool, authority_id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(user_ids))))
}

pub async fn set_role_users(
    State(state): State<AppState>,
    Json(payload): Json<system::authority::SetRoleUsersRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    system::authority::set_role_users(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("设置成功")))
}

pub async fn set_role_users_by_id(
    State(state): State<AppState>,
    Path(authority_id): Path<i64>,
    Json(mut payload): Json<system::authority::SetRoleUsersRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.authority_id = authority_id;
    system::authority::set_role_users(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("设置成功")))
}

pub async fn set_data_authority() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("设置成功"))
}
