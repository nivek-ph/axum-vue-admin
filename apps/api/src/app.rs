use axum::Router;

use crate::{router, state::AppState};

pub fn build_app() -> Router {
    router::build_router(AppState::default())
}
