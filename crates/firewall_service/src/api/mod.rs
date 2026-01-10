use crate::api::docs::ApiDoc;
use crate::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod routes;
pub mod services;
pub mod middleware;
mod docs;

pub fn app(state: AppState) -> Router {
    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());

    let public_api = Router::new()
        .merge(swagger)
        .merge(routes::public_routes());

    let secure_api = Router::new()
        .nest("/admin", routes::admin_routes())
        .nest("/users", routes::user_routes())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::auth,
        ));

    Router::new()
        .merge(public_api)
        .merge(secure_api)
        .with_state(state)
}
