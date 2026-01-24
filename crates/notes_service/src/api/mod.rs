use crate::api::docs::ApiDoc;
use crate::state::AppState;
use axum::middleware::from_fn_with_state;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use shared::auth::middleware::require_admin;

mod docs;
mod middleware;
mod routes;
mod services;

pub(crate) fn app(state: AppState) -> Router {
    let authenticated_routes = routes::authenticated_routes();
    let swagger = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi());
    let admin_routes = routes::admin_routes();

    let public_api = Router::new().merge(swagger);
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

    Router::new()
        .merge(public_api)
        .merge(secure_api)
        .merge(admin_api)
        .with_state(state)
}
