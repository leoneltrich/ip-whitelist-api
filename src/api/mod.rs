// src/api/mod.rs
use axum::Router;
use crate::persistence::repository::Repositories;

pub mod routes;
pub mod services;
pub mod middleware;

// Pass the repositories in here
pub fn app(repos: Repositories) -> Router {
    let user_routes = routes::get_routes();

    Router::new()
        .merge(user_routes)
        .with_state(repos)
}