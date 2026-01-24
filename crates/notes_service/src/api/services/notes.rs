use crate::models::api::note::{CreateNoteRequest, UpdateNoteRequest};
use crate::models::database::note::Note;
use crate::persistence::repository::interface::notes::NoteRepository;
use shared::auth::models::Claims;
use shared::errors::AppError;

pub(crate) async fn create_note(
    note_repository: &dyn NoteRepository,
    payload: &CreateNoteRequest,
    claims: &Claims,
) -> Result<(), AppError> {
    todo!()
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
    todo!()
}

pub(crate) async fn delete_all_notes_of_user(
    note_repository: &dyn NoteRepository,
    user_id: String,
    claims: &Claims,
) -> Result<usize, AppError> {
    if claims.is_admin || user_id == claims.sub {
        let result = note_repository
            .delete_all_notes_of_user(&user_id)
            .await
            .map_err(|_| {
                AppError::InternalServerError(
                    "An error occurred deleting the users notes".to_string(),
                )
            });
        return Ok(result?);
    }
    Err(AppError::Forbidden)
}
