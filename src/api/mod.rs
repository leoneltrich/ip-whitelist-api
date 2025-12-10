// src/api/mod.rs
use axum::Router;
use crate::persistence::repository::Repositories;
use crate::state::AppState;

pub mod routes;
pub mod services;
pub mod middleware;

// Pass the repositories in here
pub fn app(state: AppState) -> Router {
    // 1. Define the protected layer
    let protected = routes::protected_routes()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::auth,
        ));

    // 2. Define the public layer
    let public = routes::public_routes();

    // 3. Merge them
    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)
}