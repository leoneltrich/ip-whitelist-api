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

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/profile", put(user::self_update_user))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users", put(user::admin_update_user)) // <--- Use the Admin Handler
        .route("/users/{username}", delete(user::delete_user))
        .layer(axum::middleware::from_fn(require_admin))
}