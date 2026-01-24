use crate::models::api::note::{CreateNoteRequest, UpdateNoteRequest};
use crate::models::database::note::{NewNote, Note};
use crate::persistence::repository::interface::notes::NoteRepository;
use shared::auth::models::Claims;
use shared::errors::AppError;

pub(crate) async fn create_note(
    note_repository: &dyn NoteRepository,
    payload: &CreateNoteRequest,
    claims: &Claims,
) -> Result<i64, AppError> {
    let timestamp = chrono::Utc::now().timestamp();
    let note = NewNote {
        owner_id: claims.sub.clone(),
        is_public_read: payload.is_public_read,
        is_public_write: payload.is_public_write,
        title: payload.title.clone(),
        content: payload.content.clone(),
        timestamp_created: timestamp,
        timestamp_modified: timestamp,
    };

    let note_id = note_repository.create_note(&note).await.map_err(|_| {
        AppError::InternalServerError(
            "An internal server error occurred creating the note".to_string(),
        )
    })?;

    Ok(note_id)
}

pub(crate) async fn get_all_notes(
    note_repository: &dyn NoteRepository,
    claims: &Claims,
) -> Result<Vec<Note>, AppError> {
    todo!()
}

pub(crate) async fn get_note_by_id(
    note_repository: &dyn NoteRepository,
    note_id: String,
    claims: &Claims,
) -> Result<Note, AppError> {
    todo!()
}

pub(crate) async fn update_note(
    note_repository: &dyn NoteRepository,
    payload: &UpdateNoteRequest,
    claims: &Claims,
) -> Result<(), AppError> {
    todo!()
}

pub(crate) async fn delete_note(
    note_repository: &dyn NoteRepository,
    note_id: String,
    claims: &Claims,
) -> Result<(), AppError> {
    let internal_error = || {
        AppError::InternalServerError(
            "An internal server error occurred deleting the note".to_string(),
        )
    };

    let note_owner = note_repository
        .get_note_owner_id(&note_id)
        .await
        .map_err(|_| internal_error())?
        .ok_or(AppError::NotFound)?;

    if !claims.is_admin && note_owner != claims.sub {
        return Err(AppError::Forbidden);
    }

    let rows_deleted = note_repository
        .delete_note(&note_id)
        .await
        .map_err(|_| internal_error())?;

    if rows_deleted == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub(crate) async fn delete_all_notes_of_user(
    note_repository: &dyn NoteRepository,
    user_id: String,
    claims: &Claims,
) -> Result<usize, AppError> {
    if !claims.is_admin && user_id != claims.sub {
        return Err(AppError::Forbidden);
    }

    let result = note_repository
        .delete_all_notes_of_user(&user_id)
        .await
        .map_err(|_| {
            AppError::InternalServerError("An error occurred deleting the users notes".to_string())
        });
    Ok(result?)
}
