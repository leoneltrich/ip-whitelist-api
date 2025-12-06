mod refresh;
mod health;

use axum::{routing::get, Router};


// This function gathers all routes from the submodules
pub fn get_routes() -> Router {
    Router::new()
        .route("/health", get(health::health_handler))
}