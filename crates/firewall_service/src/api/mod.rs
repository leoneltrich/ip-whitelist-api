use crate::state::AppState;
use axum::Router;
use shared::auth::middleware;
// Import shared middleware

pub mod routes;
pub mod services;
// pub mod middleware; // Removed as it's empty
mod docs;

pub fn app(state: AppState) -> Router {
    let public_api = Router::new().merge(routes::public_routes());

    let doc_routes = routes::docs_routes();

    let secure_api = Router::new()
        .nest("/admin", routes::admin_routes())
        .nest("/users", routes::user_routes())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::<AppState>, // Explicitly specify AppState
        ));

    let aggregated_routes = Router::new()
        .merge(public_api)
        .merge(secure_api)
        .with_state(state);

    Router::new()
        .nest("/api/v1", aggregated_routes)
        .merge(doc_routes)
}
