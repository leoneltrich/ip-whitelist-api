pub mod auth;
pub mod token;
pub mod user;

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

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/health", get(health_check))
}

pub(crate) fn docs_routes() -> Router {
    Router::new().merge(
        SwaggerUi::new("/api/v1/swagger-ui")
            .url("/api/v1/api-docs/openapi.json", ApiDoc::openapi()),
    )
}

pub fn token_routes() -> Router<AppState> {
    Router::new().route("/refresh", post(token::refresh))
}

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/profile", put(user::self_update_user))
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(user::get_all_users))
        .route("/users", post(user::create_user))
        .route("/users", put(user::admin_update_user))
        .route("/users/{username}", delete(user::delete_user))
        .layer(axum::middleware::from_fn(require_admin))
}
