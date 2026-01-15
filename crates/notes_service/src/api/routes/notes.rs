use crate::api::services::notes;
use crate::models::api::note::{
    CreateNoteRequest, NoteListResponse, SingleNoteResponse, UpdateNoteRequest,
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
    post,
    path = "/notes",
    request_body = CreateNoteRequest,
    responses(
        (status = 201, description = "Note created successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn create_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    notes::create_note(&*state.repositories.note, &payload, &claims).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "created",
            "message": "Note successfully created"
        })),
    ))
}

#[utoipa::path(
    get,
    path = "/notes",
    responses(
        (status = 200, description = "List of all notes available to the user", body = NoteListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn get_all_notes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let notes = notes::get_all_notes(&*state.repositories.note, &claims).await?;

    let response = NoteListResponse {
        status: "success".to_string(),
        data: notes,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    get,
    path = "/notes/id/{id}",
    params(
        ("id" = String, Path, description = "Note id to retrieve")
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
    Path(note_id): Path<String>,
    Extension(claims): Extension<Claims>
) -> Result<impl IntoResponse, AppError> {
    let note = notes::get_note_by_id(&*state.repositories.note, note_id, &claims).await?;

    let response = SingleNoteResponse {
        status: "success".to_string(),
        data: note,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    put,
    path = "/notes",
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
    notes::update_note(&*state.repositories.note, &payload, &claims).await?;

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
    path = "/notes/{id}",
    params(
        ("id" = String, Path, description = "Note id to delete")
    ),
    responses(
        (status = 204, description = "Note deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - User is not admin nor the owner of the note"),
        (status = 404, description = "Note not found"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn delete_note(
    State(state): State<AppState>,
    Path(note_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    notes::delete_note(&*state.repositories.note, note_id, &claims).await?;

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
    path = "/notes/user/{id}",
    params(
        ("id" = String, Path, description = "User ID whose notes should be deleted")
    ),
    responses(
        (status = 204, description = "All notes for the user deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Only an admin or the resource owner can perform this action"),
        (status = 500, description = "Internal Server Error")
    ),
    security(("jwt" = []))
)]
pub async fn delete_all_notes_of_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    notes::delete_all_notes_of_user(&*state.repositories.note, user_id, &claims).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!({
            "status": "deleted",
            "message": "All notes successfully deleted"
        })),
    ))
}
