use axum::{Json, extract::State};

use super::dto::{MenuData, MenuResponse, MenuTreeData};
use crate::{ApiResponse, AppResult, extractors::current_access::CurrentAccess, state::AppState};

#[utoipa::path(
    get,
    path = "/menus/current",
    tag = "menu",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Current user menus", body = ApiResponse<MenuData>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_menu(
    State(state): State<AppState>,
    CurrentAccess(snapshot): CurrentAccess,
) -> AppResult<Json<ApiResponse<MenuData>>> {
    let (menus, permissions) = state.menus.current(snapshot).await?;
    let menus = menus
        .into_iter()
        .map(MenuResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(MenuData { menus, permissions })))
}

#[utoipa::path(
    get,
    path = "/menus/tree",
    tag = "menu",
    security(("bearer_auth" = [])),
    responses((status = 200, description = "Menu tree", body = ApiResponse<MenuTreeData>))
)]
pub async fn get_base_menu_tree(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<MenuTreeData>>> {
    let menus = state
        .menus
        .tree()
        .await?
        .into_iter()
        .map(MenuResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(MenuTreeData { menus })))
}
