use admin_httpz::{ApiResponse, AppResult, OptionAppExt};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde_json::Value;

use crate::state::AppState;

use super::dto::{
    DictionaryListQuery, DictionaryPayload, DictionaryResponse, DictionaryWithDetailsResponse,
    IdRequest, ImportDictionaryPayload,
};
use super::error::{self as errors, map_error};
use crate::routes::dictionary_details;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_sys_dictionary_list).post(create_sys_dictionary),
        )
        .route("/import", post(import_sys_dictionary))
        .route(
            "/{id}",
            get(find_sys_dictionary_by_id)
                .put(update_sys_dictionary_by_id)
                .delete(delete_sys_dictionary_by_id),
        )
        .route("/{id}/export", get(export_sys_dictionary_by_id))
        .route(
            "/{id}/details/tree",
            get(dictionary_details::get_dictionary_tree_list_by_id),
        )
}

pub async fn create_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<DictionaryPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .dictionaries
        .create(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("created")))
}

pub async fn update_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<DictionaryPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .dictionaries
        .update(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn update_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(mut payload): Json<DictionaryPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    payload.id = id;
    state
        .dictionaries
        .update(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("updated")))
}

pub async fn find_sys_dictionary(
    State(state): State<AppState>,
    Query(payload): Query<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let item = state
        .dictionaries
        .find(payload.id, payload.dictionary_type)
        .await
        .map_err(map_error)?
        .map(DictionaryWithDetailsResponse::from);
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "resysDictionary": item.map(|value| serde_json::json!(value)).unwrap_or_else(|| serde_json::json!({}))
    }))))
}

pub async fn find_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let item = state
        .dictionaries
        .find(Some(id), None)
        .await
        .map_err(map_error)?
        .map(DictionaryWithDetailsResponse::from);
    Ok(Json(ApiResponse::ok(serde_json::json!({
        "resysDictionary": item.map(|value| serde_json::json!(value)).unwrap_or_else(|| serde_json::json!({}))
    }))))
}

pub async fn get_sys_dictionary_list(
    State(state): State<AppState>,
    Query(payload): Query<DictionaryListQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let list = state
        .dictionaries
        .list(payload.into())
        .await
        .map_err(map_error)?
        .into_iter()
        .map(DictionaryResponse::from)
        .collect::<Vec<_>>();
    Ok(Json(ApiResponse::ok(serde_json::json!(list))))
}

pub async fn delete_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    state.dictionaries.delete(id).await.map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.dictionaries.delete(id).await.map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn export_sys_dictionary(
    State(state): State<AppState>,
    Query(payload): Query<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let id = payload.id.ok_or_spec(errors::ID_REQUIRED)?;
    let data = state.dictionaries.export(id).await.map_err(map_error)?;
    Ok(Json(ApiResponse::ok(
        data.unwrap_or_else(|| serde_json::json!({})),
    )))
}

pub async fn export_sys_dictionary_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let data = state.dictionaries.export(id).await.map_err(map_error)?;
    Ok(Json(ApiResponse::ok(
        data.unwrap_or_else(|| serde_json::json!({})),
    )))
}

pub async fn import_sys_dictionary(
    State(state): State<AppState>,
    Json(payload): Json<ImportDictionaryPayload>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .dictionaries
        .import(payload.into())
        .await
        .map_err(map_error)?;
    Ok(Json(ApiResponse::ok_message("imported")))
}
