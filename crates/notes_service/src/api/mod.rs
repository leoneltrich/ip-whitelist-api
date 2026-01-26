use crate::state::AppState;
use axum::middleware::from_fn_with_state;
use axum::Router;
use shared::auth::middleware::require_admin;

mod docs;
mod middleware;
mod routes;
mod services;

pub(crate) fn app(state: AppState) -> Router {
    let public_routes = routes::public_routes();
    let docs_routes = routes::docs_routes();
    let authenticated_routes = routes::authenticated_routes();
    let admin_routes = routes::admin_routes();

    let public_api = Router::new().merge(public_routes);
    let secure_api = Router::new()
        .merge(authenticated_routes)
        .layer(from_fn_with_state(
            state.clone(),
            shared::auth::middleware::auth::<AppState>,
        ));
    let admin_api = Router::new()
        .merge(admin_routes)
        .layer(axum::middleware::from_fn(require_admin))
        .layer(from_fn_with_state(
            state.clone(),
            shared::auth::middleware::auth::<AppState>,
        ));

    let aggregated_routes = Router::new()
        .merge(public_api)
        .merge(secure_api)
        .merge(admin_api)
        .with_state(state);

    Router::new()
        .nest("/api/v1", aggregated_routes)
        .merge(docs_routes)
}
