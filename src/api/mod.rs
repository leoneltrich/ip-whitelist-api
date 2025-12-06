// src/api/mod.rs
use axum::Router; // <-- Removed 'middleware' from here to fix the conflict

// Your local modules
pub mod middleware;
pub mod routes;
pub mod services;

pub fn app() -> Router {
    let my_routes = routes::get_routes();

    Router::new()
        .merge(my_routes)
        // Fix: Use the full path 'axum::middleware' here
        // The second 'self::middleware' refers to your local folder
        .layer(axum::middleware::from_fn(middleware::auth::auth))
}