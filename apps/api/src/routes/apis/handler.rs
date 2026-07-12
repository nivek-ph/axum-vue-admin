use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use serde_json::Value;

use super::dto::{
    ApiIdRequest, ApiItemResponse, ApiPayload, ApiRoleMatrixResponse, ApiRoleQuery,
    ApiRoleSelectionResponse, AuthorityApiQuery, DeleteApisByIdsRequest, SearchApiRequest,
    SetApiRolesRequest,
};
use super::error::map_error;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_api_list_by_query).post(create_api))
        .route("/all", get(get_all_apis))
        .route("/groups", get(get_api_groups))
        .route("/casbin/refresh", post(fresh_casbin))
        .route("/authority", get(get_authority_apis))
        .route("/role-matrix", get(get_api_role_matrix))
        .route("/roles", get(get_api_roles).put(set_api_roles))
        .route(
            "/{id}",
            get(get_api_by_path_id)
                .put(update_api_by_id)
                .delete(delete_api_by_path_id),
        )
        .route("/batch", delete(delete_apis_by_ids))
}

pub async fn get_api_list(
    State(state): State<AppState>,
    Json(payload): Json<SearchApiRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = state.apis.list(payload.into()).await.map_err(map_error)?;
    let list = list
        .into_iter()
        .map(ApiItemResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn get_api_list_by_query(
    State(state): State<AppState>,
    Query(payload): Query<SearchApiRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = state.apis.list(payload.into()).await.map_err(map_error)?;
    let list = list
        .into_iter()
        .map(ApiItemResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn create_api(
    State(state): State<AppState>,
    Json(payload): Json<ApiPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.apis.create(payload.into()).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("created")))
}

pub async fn get_api_by_id(
    State(state): State<AppState>,
    Json(payload): Json<ApiIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let api_record = state.apis.find(payload.id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "api": ApiItemResponse::from(api_record),
    }))))
}

pub async fn get_api_by_path_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let api_record = state.apis.find(id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "api": ApiItemResponse::from(api_record),
    }))))
}

pub async fn update_api(
    State(state): State<AppState>,
    Json(payload): Json<ApiPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.apis.update(payload.into()).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn update_api_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<ApiPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.id = id;
    state.apis.update(payload.into()).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn delete_api(
    State(state): State<AppState>,
    Json(payload): Json<ApiIdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.apis.delete(payload.id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_api_by_path_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.apis.delete(id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_apis_by_ids(
    State(state): State<AppState>,
    Json(payload): Json<DeleteApisByIdsRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .apis
        .delete_many(iam::apis::DeleteApisByIdsRequest { ids: payload.ids })
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn fresh_casbin() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("refreshed"))
}

pub async fn sync_api() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok(serde_json::json!({
        "newApis": [],
        "deleteApis": [],
        "ignoreApis": [],
    })))
}

pub async fn get_api_groups(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let (groups, api_group_map) = state.apis.groups().await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "groups": groups,
        "apiGroupMap": api_group_map,
    }))))
}

pub async fn ignore_api() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("ignored"))
}

pub async fn enter_sync_api() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::ok_message("synced"))
}

pub async fn get_all_apis(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let apis = state
        .apis
        .all()
        .await
        .map_err(map_error)?
        .into_iter()
        .map(ApiItemResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({ "apis": apis }))))
}

pub async fn get_api_roles(
    State(state): State<AppState>,
    Query(payload): Query<ApiRoleQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let selection = ApiRoleSelectionResponse::from(
        state
            .apis
            .roles(iam::apis::ApiRoleQuery {
                path: payload.path,
                method: payload.method,
            })
            .await
            .map_err(map_error)?,
    );

    Ok(Json(ApiResponse::ok(serde_json::json!(selection))))
}

pub async fn get_authority_apis(
    State(state): State<AppState>,
    Query(payload): Query<AuthorityApiQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let apis = state
        .apis
        .by_authority(payload.authority_id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(ApiItemResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "apis": apis,
    }))))
}

pub async fn get_api_role_matrix(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let items = state
        .apis
        .role_matrix()
        .await
        .map_err(map_error)?
        .into_iter()
        .map(ApiRoleMatrixResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "items": items,
    }))))
}

pub async fn set_api_roles(
    State(state): State<AppState>,
    Json(payload): Json<SetApiRolesRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .apis
        .set_roles(iam::apis::SetApiRolesRequest {
            path: payload.path,
            method: payload.method,
            authority_ids: payload.authority_ids,
        })
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("assigned")))
}
