// src/api/mod.rs
use axum::Router;
use crate::persistence::repository::Repositories;
use crate::state::AppState;

pub mod routes;
pub mod services;
pub mod middleware;

// Pass the repositories in here
pub fn app(state: AppState) -> Router {
    let routes = routes::get_routes();

    Router::new()
        .merge(routes)
        .with_state(state)
}