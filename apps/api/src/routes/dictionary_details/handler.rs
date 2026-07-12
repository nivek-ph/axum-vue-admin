use admin_httpz::{ApiResponse, AppResult, OptionAppExt};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde_json::Value;

use super::dto::{
    DictionaryDetailPayload, DictionaryDetailResponse, DictionaryParentQuery, DictionaryTreeQuery,
    DictionaryTypeQuery, IdRequest,
};
use super::error::{self as errors, map_error};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_sys_dictionary_detail))
        .route("/tree-by-type", get(get_dictionary_tree_list_by_type))
        .route("/by-parent", get(get_dictionary_details_by_parent))
        .route(
            "/{id}",
            get(find_sys_dictionary_detail_by_id)
                .put(update_sys_dictionary_detail_by_id)
                .delete(delete_sys_dictionary_detail_by_id),
        )
        .route("/{id}/path", get(get_dictionary_path_by_id))
}

pub async fn create_sys_dictionary_detail(
    State(state): State<AppState>,
    Json(payload): Json<DictionaryDetailPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .dictionaries
        .create_detail(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("created")))
}

pub async fn update_sys_dictionary_detail(
    State(state): State<AppState>,
    Json(payload): Json<DictionaryDetailPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .dictionaries
        .update_detail(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn update_sys_dictionary_detail_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<DictionaryDetailPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.id = id;
    state
        .dictionaries
        .update_detail(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn find_sys_dictionary_detail(
    State(state): State<AppState>,
    Query(payload): Query<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    let item = state
        .dictionaries
        .find_detail(id)
        .await
        .map_err(map_error)?
        .map(DictionaryDetailResponse::from);
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "reSysDictionaryDetail": item
    }))))
}

pub async fn find_sys_dictionary_detail_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let item = state
        .dictionaries
        .find_detail(id)
        .await
        .map_err(map_error)?
        .map(DictionaryDetailResponse::from);
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "reSysDictionaryDetail": item
    }))))
}

pub async fn get_dictionary_tree_list(
    State(state): State<AppState>,
    Query(payload): Query<DictionaryTreeQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .tree_by_dictionary(payload.dictionary_id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list
    }))))
}

pub async fn get_dictionary_tree_list_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .tree_by_dictionary(id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list
    }))))
}

pub async fn get_sys_dictionary_detail_list(
    State(state): State<AppState>,
    Query(payload): Query<DictionaryTreeQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .tree_by_dictionary(payload.dictionary_id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn get_dictionary_tree_list_by_type(
    State(state): State<AppState>,
    Query(payload): Query<DictionaryTypeQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .tree_by_type(&payload.dictionary_type)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn get_dictionary_details_by_parent(
    State(state): State<AppState>,
    Query(payload): Query<DictionaryParentQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .details_by_parent(payload.parent_id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn get_dictionary_path(
    State(state): State<AppState>,
    Query(payload): Query<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    let list = state
        .dictionaries
        .detail_path(id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn get_dictionary_path_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .detail_path(id)
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryDetailResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!({ "list": list }))))
}

pub async fn delete_sys_dictionary_detail(
    State(state): State<AppState>,
    Json(payload): Json<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    state
        .dictionaries
        .delete_detail(id)
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_sys_dictionary_detail_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .dictionaries
        .delete_detail(id)
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}
