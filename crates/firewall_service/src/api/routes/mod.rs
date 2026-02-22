pub mod access;
pub mod server;

use crate::api::docs::ApiDoc;
use crate::state::AppState;
use axum::routing::get;
use axum::{
    routing::{delete, post, put},
    Router,
};
use shared::auth::middleware::require_admin;
use shared::health::routes::health_check;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub(crate) fn public_routes() -> Router<AppState> {
    Router::new().route("/health", get(|| health_check(env!("CARGO_PKG_VERSION"))))
}

pub(crate) fn docs_routes() -> Router {
    Router::new().merge(
        SwaggerUi::new("/api/v1/swagger-ui")
            .url("/api/v1/api-docs/openapi.json", ApiDoc::openapi()),
    )
}

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/access", post(access::request_access))
        .route("/access/{server}/status", get(access::check_access_status))
        .route("/servers/{name}/exists", get(server::check_server_exists))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/servers", post(server::create_server))
        .route("/servers", get(server::list_servers))
        .route("/servers/{name}", get(server::get_server))
        .route("/servers/{name}", put(server::update_server))
        .route("/servers/{name}", delete(server::delete_server))
        .layer(axum::middleware::from_fn(require_admin))
}
