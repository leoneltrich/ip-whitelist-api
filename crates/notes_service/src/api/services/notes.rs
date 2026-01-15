use shared::errors::AppError;
use crate::models::database::note::Note;
use crate::state::AppState;

pub(crate) async fn create_note() -> Result<(), AppError>{
    todo!()
}

pub(crate) async fn get_all_notes() -> Result<Vec<Note>, AppError> {
    todo!()
}

pub(crate) async fn get_note_by_id() -> Result<Note, AppError>{
    todo!()
}

pub(crate) async fn update_note() -> Result<(), AppError>{
    todo!()
}

pub(crate) async fn delete_note() -> Result<(), AppError> {
    todo!()
}

pub(crate) async fn delete_all_notes_of_user() -> Result<(), AppError> {
    todo!()
}