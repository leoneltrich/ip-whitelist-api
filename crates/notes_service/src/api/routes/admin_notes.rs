use crate::api::services::notes;
use crate::models::api::note::{
    NoteListResponse, SingleNoteResponse, UpdateNoteRequest,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State}, http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use serde_json::json;
use shared::auth::models::Claims;
use shared::errors::AppError;

#[utoipa::path(
    get,
    path = "/admin/notes",
    responses(
        (status = 200, description = "List of all notes", body = NoteListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn get_all_notes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let notes = notes::get_notes_feed_as_admin(&*state.repositories.note).await?;

    let response = NoteListResponse {
        status: "success".to_string(),
        data: notes,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    get,
    path = "/admin/notes/id/{id}",
    params(
        ("id" = i64, Path, description = "Note id to retrieve")
    ),
    responses(
        (status = 200, description = "The retrieved note", body = SingleNoteResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User does not have permission to view the note"),
        (status = 404, description = "Note not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn get_note_by_id(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
    Extension(claims): Extension<Claims>
) -> Result<impl IntoResponse, AppError> {
    let note = notes::get_note_by_id_as_admin(&*state.repositories.note, note_id).await?;

    let response = SingleNoteResponse {
        status: "success".to_string(),
        data: note,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    put,
    path = "/admin/notes",
    request_body = UpdateNoteRequest,
    responses(
        (status = 200, description = "Note updated successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User does not have permission to update the note"),
        (status = 404, description = "Note not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn update_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateNoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    notes::update_note_as_admin(&*state.repositories.note, &payload).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Note updated successfully"
        })),
    ))
}

#[utoipa::path(
    delete,
    path = "/admin/notes/{id}",
    params(
        ("id" = i64, Path, description = "Note id to delete")
    ),
    responses(
        (status = 204, description = "Note deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not admin"),
        (status = 404, description = "Note not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn delete_note(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    notes::delete_note_as_admin(&*state.repositories.note, note_id).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!({
            "status": "deleted",
            "message": "Note successfully deleted"
        })),
    ))
}

#[utoipa::path(
    delete,
    path = "/admin/notes/user/{id}",
    params(
        ("id" = String, Path, description = "User ID whose notes should be deleted")
    ),
    responses(
        (status = 204, description = "All notes for the user deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Only an admin can perform this action"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn delete_all_notes_of_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    notes::delete_all_notes_as_admin(&*state.repositories.note, user_id).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!({
            "status": "deleted",
            "message": "All notes successfully deleted"
        })),
    ))
}
