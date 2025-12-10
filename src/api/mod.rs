// src/api/mod.rs
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::api::docs::ApiDoc;
use crate::persistence::repository::Repositories;
use crate::state::AppState;

pub mod routes;
pub mod services;
pub mod middleware;
mod docs;

// Pass the repositories in here
pub fn app(state: AppState) -> Router {

    let users = routes::user_routes();
    let admin = routes::admin_routes();

    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    let public_api = Router::new()
        .merge(swagger)                   // Swagger is public
        .merge(routes::public_routes());

    let secure_api = Router::new()
        .nest("/admin", admin)
        .nest("/users", users)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::auth,
        ));

    // 3. Final Assembly
    Router::new()
        .merge(public_api)
        .merge(secure_api)
        .with_state(state)
}