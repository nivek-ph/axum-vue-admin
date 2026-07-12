use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde_json::Value;

use super::dto::{MenuIdRequest, MenuPayload, MenuRoleSelectionResponse, SetMenuRolesRequest};
use super::error::map_error;
use crate::{extractors::current_user::CurrentUser, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/current", get(get_menu))
        .route("/", get(get_menu_list).post(add_base_menu))
        .route("/tree", get(get_base_menu_tree))
        .route(
            "/{id}",
            get(get_base_menu_by_path_id)
                .put(update_base_menu_by_id)
                .delete(delete_base_menu_by_id),
        )
        .route(
            "/{id}/roles",
            get(get_menu_roles_by_id).put(set_menu_roles_by_id),
        )
}

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
) -> AppResult<Json<ApiResponse<Value>>> {
    let menus = state
        .menus
        .for_user(user.id, user.authority_id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(MenuPayload::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menus": menus,
    }))))
}

pub async fn get_menu_list(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let menus = state
        .menus
        .list()
        .await
        .map_err(map_error)?
        .into_iter()
        .map(MenuPayload::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!(menus))))
}

pub async fn get_base_menu_tree(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let menus = state
        .menus
        .tree()
        .await
        .map_err(map_error)?
        .into_iter()
        .map(MenuPayload::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menus": menus,
    }))))
}

pub async fn add_base_menu(
    State(state): State<AppState>,
    Json(payload): Json<MenuPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .menus
        .create(payload.into())
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("created")))
}

pub async fn update_base_menu(
    State(state): State<AppState>,
    Json(payload): Json<MenuPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .menus
        .update(payload.into())
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn update_base_menu_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<MenuPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.id = id;
    state
        .menus
        .update(payload.into())
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn delete_base_menu(
    State(state): State<AppState>,
    Json(payload): Json<MenuIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.menus.delete(payload.id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_base_menu_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.menus.delete(id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn get_base_menu_by_id(
    State(state): State<AppState>,
    Json(payload): Json<MenuIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let menu = MenuPayload::from(state.menus.find(payload.id).await.map_err(map_error)?);

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menu": menu,
    }))))
}

pub async fn get_base_menu_by_path_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let menu = MenuPayload::from(state.menus.find(id).await.map_err(map_error)?);

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "menu": menu,
    }))))
}

pub async fn get_menu_roles(
    State(state): State<AppState>,
    Query(payload): Query<MenuIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let data =
        MenuRoleSelectionResponse::from(state.menus.roles(payload.id).await.map_err(map_error)?);

    Ok(Json(ApiResponse::ok(serde_json::json!(data))))
}

pub async fn get_menu_roles_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let data = MenuRoleSelectionResponse::from(state.menus.roles(id).await.map_err(map_error)?);

    Ok(Json(ApiResponse::ok(serde_json::json!(data))))
}

pub async fn set_menu_roles(
    State(state): State<AppState>,
    Json(payload): Json<SetMenuRolesRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .menus
        .set_roles(menu::SetMenuRolesRequest {
            menu_id: payload.menu_id,
            authority_ids: payload.authority_ids,
        })
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("assigned")))
}

pub async fn set_menu_roles_by_id(
    State(state): State<AppState>,
    Path(menu_id): Path<i64>,
    Json(mut payload): Json<SetMenuRolesRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.menu_id = menu_id;
    state
        .menus
        .set_roles(menu::SetMenuRolesRequest {
            menu_id: payload.menu_id,
            authority_ids: payload.authority_ids,
        })
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("assigned")))
}
