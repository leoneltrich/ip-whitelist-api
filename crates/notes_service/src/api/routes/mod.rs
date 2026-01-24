use axum::Router;
use axum::routing::{delete, get, post, put};
use crate::state::AppState;

pub(crate) mod user_notes;
pub(crate) mod admin_notes;

pub(crate) fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/notes", post(user_notes::create_note))
        .route("/notes", get(user_notes::get_all_notes))
        .route("/notes/id/{id}", get(user_notes::get_note_by_id))
        .route("/notes", put(user_notes::update_note))
        .route("/notes/{id}", delete(user_notes::delete_note))
        .route("/notes/user/{id}", delete(user_notes::delete_all_notes_of_user))
}

pub(crate) fn admin_routes() -> Router<AppState> {
    Router::new()
}