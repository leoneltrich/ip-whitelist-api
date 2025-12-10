mod refresh;
mod health;
mod user;
mod auth;

use crate::state::AppState;
use axum::{
    routing::{delete, post, put},
    Router,
};
use crate::api::middleware::auth::require_admin;

// Public routes (Login, Register)
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
}

// Protected routes (Update, Delete)
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/users", put(user::update_user))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users/{username}", delete(user::delete_user))
        .layer(axum::middleware::from_fn(require_admin))
}