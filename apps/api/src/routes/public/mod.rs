pub mod auth;
pub mod captcha;
pub mod health;
pub mod init;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<crate::state::AppState> {
    Router::new()
        .route("/health", get(health::health))
        .route("/auth/login", post(auth::login))
        .route("/auth/captcha", post(captcha::captcha))
        .route("/init/check-db", post(init::check_db))
        .route("/init/database", post(init::init_db))
}
