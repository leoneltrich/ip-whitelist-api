use shared::errors::app_errors::AppError;
use crate::persistence::repository::interface::notes::NoteRepository;

pub(crate) fn get_deletion_error() -> AppError {
    AppError::InternalServerError("An internal server error occurred deleting the note".to_string())
}

pub async fn get_note_owner(note_repository: &dyn NoteRepository, note_id: &i64) -> Result<String, AppError> {
    let note_owner = note_repository
        .get_note_owner_id(note_id)
        .await
        .map_err(|_| get_deletion_error())?
        .ok_or(AppError::NotFound)?;
    Ok(note_owner)
}