use crate::api::services::utils;
use crate::api::services::utils::get_note_owner;
use crate::models::api::note::{CreateNoteRequest, UpdateNoteRequest};
use crate::models::database::note::{NewNote, Note, UpdateNote};
use crate::persistence::repository::interface::notes::NoteRepository;
use log::{error, info, warn};
use shared::auth::models::Claims;
use shared::errors::app_errors::AppError;
use tracing::debug;

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

    let note_id = note_repository.create_note(&note).await.map_err(|e| {
        error!("An error occurred creating the note: {}", e);
        AppError::InternalServerError("An internal server error occurred".to_string())
    })?;

    info!("Note created successfully with id: {}", note_id);
    Ok(note_id)
}

pub(crate) async fn get_notes_feed_as_admin(
    note_repository: &dyn NoteRepository,
) -> Result<Vec<Note>, AppError> {
    debug!("Getting notes feed for admin user");
    note_repository.get_all_notes_feed().await.map_err(|e| {
        error!("An error occurred, could not get notes feed: {}", e);
        AppError::InternalServerError("An internal server error occurred".to_string())
    })
}

pub(crate) async fn get_own_notes_feed(
    note_repository: &dyn NoteRepository,
    claims: &Claims,
) -> Result<Vec<Note>, AppError> {
    debug!("Getting users own notes");
    note_repository
        .get_notes_feed(&claims.sub)
        .await
        .map_err(|e| {
            error!("An error occurred getting users notes: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })
}

pub(crate) async fn get_note_by_id_as_admin(
    note_repository: &dyn NoteRepository,
    note_id: i64,
) -> Result<Option<Note>, AppError> {
    debug!("Getting note by id: {} as admin", note_id);
    get_note_by_id(note_repository, note_id).await
}

pub(crate) async fn get_own_note_by_id(
    note_repository: &dyn NoteRepository,
    note_id: i64,
    claims: &Claims,
) -> Result<Option<Note>, AppError> {
    let owner = get_note_owner(note_repository, &note_id).await?;

    if owner != claims.sub {
        info!(
            "User {} trying to access note {} that is not owned by them",
            claims.sub, note_id
        );
        return Err(AppError::PermissionDenied);
    }

    debug!("Getting note by id: {} for user {}", note_id, claims.sub);
    get_note_by_id(note_repository, note_id).await
}

async fn get_note_by_id(
    note_repository: &dyn NoteRepository,
    note_id: i64,
) -> Result<Option<Note>, AppError> {
    note_repository.get_note_by_id(&note_id).await.map_err(|e| {
        error!(
            "An error occurred getting the note with id: {}, Error: {}",
            note_id, e
        );
        AppError::InternalServerError(
            "An internal server error occurred getting the note".to_string(),
        )
    })
}

pub(crate) async fn update_own_note(
    note_repository: &dyn NoteRepository,
    payload: &UpdateNoteRequest,
    claims: &Claims,
) -> Result<usize, AppError> {
    let note_owner = get_note_owner(note_repository, &payload.id).await?;

    if note_owner != claims.sub {
        info!(
            "User {} is trying to update a note that's not theirs",
            claims.sub
        );
        return Err(AppError::PermissionDenied);
    }

    info!("Updating note with id {}", payload.id);
    update_note(note_repository, &payload).await
}

pub(crate) async fn update_note_as_admin(
    note_repository: &dyn NoteRepository,
    payload: &UpdateNoteRequest,
) -> Result<usize, AppError> {
    update_note(note_repository, &payload).await
}

async fn update_note(
    note_repository: &dyn NoteRepository,
    payload: &UpdateNoteRequest,
) -> Result<usize, AppError> {
    let timestamp = chrono::Utc::now().timestamp();
    let updated_note = UpdateNote {
        note_id: payload.id,
        is_public_read: payload.is_public_read,
        is_public_write: payload.is_public_write,
        title: payload.title.clone(),
        content: payload.content.clone(),
        timestamp_modified: timestamp,
    };

    note_repository
        .update_note(&updated_note)
        .await
        .map_err(|e| {
            error!("An error occurred updating note: {}", e);
            AppError::InternalServerError(
                "An internal server error occurred updating note".to_string(),
            )
        })
}

pub(crate) async fn delete_note_as_admin(
    note_repository: &dyn NoteRepository,
    note_id: i64,
) -> Result<(), AppError> {
    debug!("Deleting note with id {} as admin", note_id);
    delete_note(note_repository, note_id).await
}

pub(crate) async fn delete_own_note(
    note_repository: &dyn NoteRepository,
    note_id: i64,
    claims: &Claims,
) -> Result<(), AppError> {
    let note_owner = get_note_owner(note_repository, &note_id).await?;

    if note_owner != claims.sub {
        info!(
            "User {} is trying to delete note that isn't theirs",
            claims.sub
        );
        return Err(AppError::PermissionDenied);
    }

    info!("Deleting note with id {}", note_id);
    delete_note(note_repository, note_id).await
}

async fn delete_note(note_repository: &dyn NoteRepository, note_id: i64) -> Result<(), AppError> {
    let rows_deleted = note_repository.delete_note(&note_id).await.map_err(|e| {
        error!(
            "An error occurred deleting the note with id: {}, Error: {}",
            note_id, e
        );
        AppError::InternalServerError("An internal server error occurred".to_string())
    })?;

    if rows_deleted == 0 {
        info!("Note with id: {} not found", note_id);
        return Err(AppError::NotFound);
    }

    info!("Note with id: {} deleted successfully", note_id);
    Ok(())
}

pub(crate) async fn delete_all_notes_self(
    note_repository: &dyn NoteRepository,
    claims: &Claims,
) -> Result<usize, AppError> {
    debug!("User {} is deleting all their notes", claims.sub);
    delete_all_notes_of_user(note_repository, claims.sub.clone()).await
}

pub(crate) async fn delete_all_notes_as_admin(
    note_repository: &dyn NoteRepository,
    user_id: String,
) -> Result<usize, AppError> {
    info!(
        "Admin user {} is deleting all notes of user {}",
        user_id, user_id
    );
    delete_all_notes_of_user(note_repository, user_id).await
}

async fn delete_all_notes_of_user(
    note_repository: &dyn NoteRepository,
    user_id: String,
) -> Result<usize, AppError> {
    let result = note_repository
        .delete_all_notes_of_user(&user_id)
        .await
        .map_err(|e| {
            error!(
                "An error occurred deleting all notes of user {}: {}",
                user_id, e
            );
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?;

    info!(
        "All notes of user {} ({} notes total) deleted successfully",
        result, user_id
    );
    Ok(result)
}
