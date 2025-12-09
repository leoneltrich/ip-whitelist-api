mod refresh;
mod health;
mod user;
mod auth;

use axum::{
    routing::{post, put, delete},
    Router,
};
use crate::persistence::repository::Repositories;


pub fn get_routes() -> Router<Repositories> {
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