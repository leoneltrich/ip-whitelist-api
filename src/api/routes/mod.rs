mod refresh;
mod health;
mod user;
mod auth;

use crate::state::AppState;
use axum::{
    routing::{delete, post, put},
    Router,
};

// Public routes (Login, Register)
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
}

// Protected routes (Update, Delete)
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/users", put(user::update_user))
        .route("/users/{username}", delete(user::delete_user))
        .route("/users", post(user::create_user))
}