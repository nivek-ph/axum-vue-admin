use axum::{Json, extract::State, http::HeaderMap};

use crate::{
    ApiResponse, AppResult, NoData, mappings::LOGIN_REQUIRED,
    middleware::auth::extract_bearer_token, state::AppState,
};

#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Logout success", body = ApiResponse<NoData>),
        (status = 401, description = "Invalid or revoked session")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> AppResult<Json<ApiResponse<NoData>>> {
    let token = extract_bearer_token(&headers).ok_or(LOGIN_REQUIRED)?;
    state.tokens.revoke(token).await?;

    Ok(Json(ApiResponse::new("OK", "signed out", None)))
}
