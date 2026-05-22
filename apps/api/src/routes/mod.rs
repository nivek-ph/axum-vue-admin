pub mod protected;
pub mod public;

use axum::Router;

pub fn public_routes() -> Router<crate::state::AppState> {
    public::router()
}

pub fn protected_routes() -> Router<crate::state::AppState> {
    protected::router()
}
