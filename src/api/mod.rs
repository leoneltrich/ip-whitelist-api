// src/api/mod.rs
use axum::Router;
use crate::persistence::repository::Repositories;
use crate::state::AppState;

pub mod routes;
pub mod services;
pub mod middleware;

// Pass the repositories in here
pub fn app(state: AppState) -> Router {

    let public = routes::public_routes();
    let users = routes::user_routes();
    let admin = routes::admin_routes();

    let secure_api = Router::new()
        .nest("/admin", admin)
        .nest("/users", users)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::auth,
        ));

    // 3. Final Assembly
    Router::new()
        .merge(public)
        .merge(secure_api)
        .with_state(state)
}