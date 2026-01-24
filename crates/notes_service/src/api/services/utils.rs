use shared::errors::AppError;

pub(crate) fn get_deletion_error() -> AppError {
    AppError::InternalServerError("An internal server error occurred deleting the note".to_string())
}