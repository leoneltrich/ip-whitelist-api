pub mod access;
pub mod server;
pub mod health;

use crate::api::middleware::auth::require_admin;
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, post, put},
};
use axum::routing::get;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health_check))
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/access", post(access::request_access))
        .route("/access/{server}/status", get(access::check_access_status))
        .route("/servers/{name}/exists", get(server::check_server_exists))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/servers", post(server::create_server))
        .route("/servers", get(server::list_servers))
        .route("/servers/{name}", get(server::get_server))
        .route("/servers/{name}", put(server::update_server))
        .route("/servers/{name}", delete(server::delete_server))
        .layer(axum::middleware::from_fn(require_admin))
}
