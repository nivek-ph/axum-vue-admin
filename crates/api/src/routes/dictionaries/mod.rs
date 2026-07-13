mod details;
mod dto;
mod error;
mod handler;
pub use handler::*;

pub fn detail_routes() -> axum::Router<crate::state::AppState> {
    details::routes()
}
