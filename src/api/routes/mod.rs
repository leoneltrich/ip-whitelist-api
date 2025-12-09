mod refresh;
mod health;
mod user;
mod auth;

use crate::state::AppState;
use axum::{
    routing::{delete, post, put},
    Router,
};

pub fn get_routes() -> Router<AppState> {
    let user_routes = Router::new()
        .route("/users", post(user::create_user))
        .route("/users", put(user::update_user))
        .route("/users/{username}", delete(user::delete_user));

    let auth_routes = Router::new()
        .route("/login", post(auth::login));

    // Combine them
    Router::new()
        .merge(user_routes)
        .merge(auth_routes)
}