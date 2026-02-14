use crate::api::routes::docs_routes;
use crate::state::AppState;
// src/api/mod.rs
use axum::Router;
use shared::auth::middleware;
// Import shared middleware

pub mod routes;
pub mod services;
// pub mod middleware; // Removed as it's empty
mod docs;

// Pass the repositories in here
pub fn app(state: AppState) -> Router {
    let users = routes::user_routes();
    let admin = routes::admin_routes();

    let swagger = docs_routes();

    let public_api = Router::new()
        .merge(routes::public_routes());

    let secure_api = Router::new()
        .nest("/admin", admin)
        .nest("/users", users)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::<AppState>,
        ));

    let aggregated_routes = Router::new()
        .merge(secure_api)
        .merge(public_api)
        .with_state(state);

    Router::new()
        .nest("/api/v1", aggregated_routes)
        .merge(swagger)
}
