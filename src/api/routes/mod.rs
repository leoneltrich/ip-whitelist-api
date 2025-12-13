pub mod user;
pub mod auth;
pub mod access;
pub mod server;
pub mod token;
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
        .route("/login", post(auth::login))
        .route("/health", get(health::health_check))
}

pub fn token_routes() -> Router<AppState> {
    Router::new()
        .route("/expires", get(token::expires))
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/profile", put(user::self_update_user))
        .route("/access", post(access::request_access))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users", put(user::admin_update_user)) // <--- Use the Admin Handler
        .route("/users/{username}", delete(user::delete_user))
        .route("/servers", post(server::create_server))

        .route("/servers", get(server::list_servers))
        .route("/servers/{name}", get(server::get_server))
        .route("/servers/{name}", put(server::update_server))
        .route("/servers/{name}", delete(server::delete_server))

        .layer(axum::middleware::from_fn(require_admin))
}
