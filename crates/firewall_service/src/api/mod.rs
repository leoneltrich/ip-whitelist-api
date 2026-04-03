use crate::state::AppState;
use axum::extract::DefaultBodyLimit;
use axum::Router;
use shared::auth::middleware;
use std::sync::Arc;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;

use shared::rate_limiting::SmartIpExtractor;

pub mod routes;
pub mod services;
mod docs;

pub fn app(state: AppState) -> Router {
    let rate_limit = Arc::new(
        GovernorConfigBuilder::default()
            .per_millisecond(200)
            .burst_size(30)
            .key_extractor(SmartIpExtractor)
            .finish()
            .unwrap()
    );

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
        .layer(DefaultBodyLimit::max(4096))
        .layer(GovernorLayer::new(rate_limit.clone()))
        .merge(doc_routes)
}
