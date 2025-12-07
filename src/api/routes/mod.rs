mod refresh;
mod health;
mod user;

use axum::{
    routing::{post, put, delete},
    Router,
};
use crate::persistence::repository::Repositories;


// Note the return type change: Router<Repositories>
// This tells Axum: "This router expects 'Repositories' to exist in the state"
pub fn get_routes() -> Router<Repositories> {
    Router::new()
        .route("/users", post(user::create_user))
        .route("/users", put(user::update_user))
        .route("/users/{username}", delete(user::delete_user))}