use crate::api::routes::docs_routes;
use crate::state::AppState;
use axum::{Router, extract::DefaultBodyLimit};
use shared::auth::middleware;

pub mod routes;
pub mod services;
mod docs;

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
        .layer(DefaultBodyLimit::max(4096))
        .merge(swagger)
}
