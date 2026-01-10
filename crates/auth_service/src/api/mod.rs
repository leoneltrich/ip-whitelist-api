use crate::api::docs::ApiDoc;
use crate::state::AppState;
// src/api/mod.rs
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod routes;
pub mod services;
pub mod middleware;
mod docs;

// Pass the repositories in here
pub fn app(state: AppState) -> Router {

    let users = routes::user_routes();
    let admin = routes::admin_routes();
    let token = routes::token_routes();

    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    let public_api = Router::new()
        .merge(swagger)
        .merge(routes::public_routes());

    let secure_api = Router::new()
        .nest("/admin", admin)
        .nest("/users", users)
        .nest("/token", token)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::auth,
        ));

    Router::new()
        .merge(public_api)
        .merge(secure_api)
        .with_state(state)
}