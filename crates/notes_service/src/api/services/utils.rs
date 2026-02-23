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
            AppError::InternalServerError
        })?
        .ok_or(AppError::NotFound)?;
    Ok(note_owner)
}

pub async fn is_note_public_write(
    note_repository: &dyn NoteRepository,
    note_id: &i64,
) -> Result<bool, AppError> {
    debug!("Getting public write status for not with id: {}", note_id);
    let is_public_write = note_repository
        .get_note_by_id(note_id)
        .await
        .map_err(|_| {
            error!("An error occurred getting the note with id: {}", note_id);
            AppError::InternalServerError
        })?
        .ok_or(AppError::NotFound)?
        .is_public_write;

    Ok(is_public_write)
}

pub async fn is_note_public_read(
    note_repository: &dyn NoteRepository,
    note_id: &i64,
) -> Result<bool, AppError> {
    debug!("Getting public read status for not with id: {}", note_id);
    let is_public_write = note_repository
        .get_note_by_id(note_id)
        .await
        .map_err(|_| {
            error!("An error occurred getting the note with id: {}", note_id);
            AppError::InternalServerError
        })?
        .ok_or(AppError::NotFound)?
        .is_public_read;

    Ok(is_public_write)
}
