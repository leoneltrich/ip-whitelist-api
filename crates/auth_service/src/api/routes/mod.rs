pub mod user;
pub mod auth;
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
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::get_all_users))
        .route("/users", post(user::create_user))
        .route("/users", put(user::admin_update_user)) 
        .route("/users/{username}", delete(user::delete_user))

        .layer(axum::middleware::from_fn(require_admin))
}
