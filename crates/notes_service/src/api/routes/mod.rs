use axum::Router;
use axum::routing::{delete, get, post, put};
use crate::state::AppState;

pub(crate) mod notes;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/notes", post(notes::create_note))
        .route("/notes", get(notes::get_all_notes))
        .route("/notes/id/{id}", get(notes::get_note_by_id))
        .route("/notes", put(notes::update_note))
        .route("/notes/{id}", delete(notes::delete_note))
        .route("/notes/user/{id}", delete(notes::delete_all_notes_of_user))
}