use admin_httpz::{ApiResponse, AppError};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;

use crate::{extractors::current_user::CurrentUser, state::AppState};

#[utoipa::path(
    get,
    path = "/api/menus/current",
    tag = "menu",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Dynamic menus", body = crate::docs::MenuResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_menu(
    State(state): State<AppState>,
    CurrentUser(user): CurrentUser,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let menus = system::menu::get_menu_tree_for_authority(&state.pool, user.authority_id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menus": menus,
    }))))
}

pub async fn get_menu_list(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let menus = system::menu::get_menu_list(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(menus))))
}

pub async fn get_base_menu_tree(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let menus = system::menu::get_base_menu_tree(&state.pool).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menus": menus,
    }))))
}

pub async fn add_base_menu(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::MenuView>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::menu::add_base_menu(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("添加成功")))
}

pub async fn update_base_menu(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::MenuView>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::menu::update_base_menu(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("更新成功")))
}

pub async fn update_base_menu_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<system::menu::MenuView>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    payload.id = id;
    system::menu::update_base_menu(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("更新成功")))
}

pub async fn delete_base_menu(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::MenuIdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::menu::delete_base_menu(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn delete_base_menu_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::menu::delete_base_menu(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok_message("删除成功")))
}

pub async fn get_base_menu_by_id(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::MenuIdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let menu = system::menu::get_base_menu_by_id(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menu": menu,
    }))))
}

pub async fn get_base_menu_by_path_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let menu = system::menu::get_base_menu_by_id(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menu": menu,
    }))))
}

pub async fn get_menu_authority(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::MenuAuthorityRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let menus = system::menu::get_menu_authority(&state.pool, payload.authority_id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menus": menus,
    }))))
}

pub async fn add_menu_authority(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::AddMenuAuthorityRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::menu::add_menu_authority(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("添加成功")))
}

pub async fn get_menu_roles(
    State(state): State<AppState>,
    Query(payload): Query<system::menu::MenuIdRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let data = system::menu::get_menu_roles(&state.pool, payload.id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(data))))
}

pub async fn get_menu_roles_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    let data = system::menu::get_menu_roles(&state.pool, id).await?;

    Ok(Json(ApiResponse::ok(serde_json::json!(data))))
}

pub async fn set_menu_roles(
    State(state): State<AppState>,
    Json(payload): Json<system::menu::SetMenuRolesRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    system::menu::set_menu_roles(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("分配成功")))
}

pub async fn set_menu_roles_by_id(
    State(state): State<AppState>,
    Path(menu_id): Path<i64>,
    Json(mut payload): Json<system::menu::SetMenuRolesRequest>,
) -> Result<Json<ApiResponse<Value>>, AppError> {
    payload.menu_id = menu_id;
    system::menu::set_menu_roles(&state.pool, payload).await?;

    Ok(Json(ApiResponse::ok_message("分配成功")))
}
