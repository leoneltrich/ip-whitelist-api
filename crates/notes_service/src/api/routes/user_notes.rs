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
use shared::errors::app_errors::AppError;
use shared::errors::utoipa_errors::{
    AccessAuthErrorResponse, BadRequestErrorResponse,
    InternalServerErrorResponse, NotFoundErrorResponse, PermissionErrorResponse,
};

#[utoipa::path(
    post,
    path = "/api/v1/notes",
    request_body = CreateNoteRequest,
    responses(
        (status = 201, description = "Note created successfully"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
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
    path = "/api/v1/notes",
    responses(
        (status = 200, description = "List of all notes available to the user", body = NoteListResponse),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = []))
)]
pub async fn get_all_notes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let notes = notes::get_own_notes_feed(&*state.repositories.note, &claims).await?;

    let response = NoteListResponse {
        status: "success".to_string(),
        data: notes,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    get,
    path = "/api/v1/notes/{id}",
    params(
        ("id" = i64, Path, description = "Note id to retrieve")
    ),
    responses(
        (status = 200, description = "The retrieved note", body = SingleNoteResponse),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = []))
)]
pub async fn get_note_by_id(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let note = notes::get_own_note_by_id(&*state.repositories.note, note_id, &claims).await?;

    let response = SingleNoteResponse {
        status: "success".to_string(),
        data: note,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    put,
    path = "/api/v1/notes",
    request_body = UpdateNoteRequest,
    responses(
        (status = 200, description = "Note updated successfully"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = []))
)]
pub async fn update_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateNoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    notes::update_note_as_user(&*state.repositories.note, &payload, &claims).await?;

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
    path = "/api/v1/notes/{id}",
    params(
        ("id" = i64, Path, description = "Note id to delete")
    ),
    responses(
        (status = 204, description = "Note deleted successfully"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = []))
)]
pub async fn delete_note(
    State(state): State<AppState>,
    Path(note_id): Path<i64>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    notes::delete_own_note(&*state.repositories.note, note_id, &claims).await?;

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
    path = "/api/v1/notes/user",
    responses(
        (status = 204, description = "All notes for the user deleted"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = []))
)]
pub async fn delete_all_notes_of_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    notes::delete_all_notes_self(&*state.repositories.note, &claims).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!({
            "status": "deleted",
            "message": "All notes successfully deleted"
        })),
    ))
}
