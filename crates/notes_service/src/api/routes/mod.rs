use crate::api::docs::ApiDoc;
use crate::state::AppState;
use axum::routing::{delete, get, post, put};
use axum::Router;
use shared::health::routes::health_check;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub(crate) mod admin_notes;
pub(crate) mod user_notes;

pub(crate) fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/notes", post(user_notes::create_note))
        .route("/notes", get(user_notes::get_all_notes))
        .route("/notes/id/{id}", get(user_notes::get_note_by_id))
        .route("/notes", put(user_notes::update_note))
        .route("/notes/{id}", delete(user_notes::delete_note))
        .route(
            "/notes/user/{id}",
            delete(user_notes::delete_all_notes_of_user),
        )
}

pub(crate) fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/admin/notes", get(admin_notes::get_all_notes))
        .route("/admin/notes/id/{id}", get(admin_notes::get_note_by_id))
        .route("/admin/notes", put(admin_notes::update_note))
        .route("/admin/notes/{id}", delete(admin_notes::delete_note))
        .route(
            "/admin/notes/user/{id}",
            delete(admin_notes::delete_all_notes_of_user),
        )
}

pub(crate) fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/notes/health", get(health_check))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
