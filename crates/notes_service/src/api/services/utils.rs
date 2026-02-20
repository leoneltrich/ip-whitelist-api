use crate::persistence::repository::interface::notes::NoteRepository;
use shared::errors::app_errors::AppError;
use tracing::error;
use tracing::log::debug;

pub async fn get_note_owner(
    note_repository: &dyn NoteRepository,
    note_id: &i64,
) -> Result<String, AppError> {
    debug!("Getting note owner of note with id: {}", note_id);
    let note_owner = note_repository
        .get_note_owner_id(note_id)
        .await
        .map_err(|e| {
            error!(
                "An error occurred getting the note owner of note with id: {}, Error: {}",
                note_id, e
            );
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?
        .ok_or(AppError::NotFound)?;
    Ok(note_owner)
}
