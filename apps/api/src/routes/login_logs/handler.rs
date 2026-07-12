use admin_httpz::{ApiResponse, AppResult};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde_json::Value;

use super::dto::{IdRequest, IdsRequest, LoginLogResponse, LoginLogSearch};
use super::error::map_error;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_login_log_list).delete(delete_login_log_by_ids))
        .route(
            "/{id}",
            get(find_login_log_by_id).delete(delete_login_log_by_id),
        )
}

pub async fn get_login_log_list(
    State(state): State<AppState>,
    Query(payload): Query<LoginLogSearch>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let page = payload.page.max(1);
    let page_size = payload.page_size.max(1);
    let (list, total) = state
        .login_logs
        .list(payload.into())
        .await
        .map_err(map_error)?;
    let list = list
        .into_iter()
        .map(LoginLogResponse::from)
        .collect::<Vec<_>>();

    Ok(Json(ApiResponse::ok(serde_json::json!({
        "list": list,
        "total": total,
        "page": page,
        "pageSize": page_size,
    }))))
}

pub async fn find_login_log(
    State(state): State<AppState>,
    Query(payload): Query<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let (list, _) = state
        .login_logs
        .list(audit::login_logs::LoginLogSearch {
            page: 1,
            page_size: 9999,
            username: None,
            status: None,
        })
        .await
        .map_err(map_error)?;
    let item = list
        .into_iter()
        .find(|log| log.id == payload.id)
        .map(LoginLogResponse::from);
    Ok(Json(ApiResponse::ok(match item {
        Some(log) => serde_json::json!(log),
        None => serde_json::json!({}),
    })))
}

pub async fn find_login_log_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let (list, _) = state
        .login_logs
        .list(audit::login_logs::LoginLogSearch {
            page: 1,
            page_size: 9999,
            username: None,
            status: None,
        })
        .await
        .map_err(map_error)?;
    let item = list
        .into_iter()
        .find(|log| log.id == id)
        .map(LoginLogResponse::from);
    Ok(Json(ApiResponse::ok(match item {
        Some(log) => serde_json::json!(log),
        None => serde_json::json!({}),
    })))
}

pub async fn delete_login_log(
    State(state): State<AppState>,
    Json(payload): Json<IdRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .login_logs
        .delete(payload.id)
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_login_log_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state.login_logs.delete(id).await.map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("deleted")))
}

pub async fn delete_login_log_by_ids(
    State(state): State<AppState>,
    Json(payload): Json<IdsRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    state
        .login_logs
        .delete_many(payload.ids)
        .await
        .map_err(map_error)?;

    Ok(Json(ApiResponse::ok_message("batch deleted")))
}
