use crate::api::services::utils;
use crate::api::services::utils::get_note_owner;
use crate::models::api::note::{CreateNoteRequest, UpdateNoteRequest};
use crate::models::database::note::{NewNote, Note, UpdateNote};
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

pub(crate) async fn get_notes_feed_as_admin(
    note_repository: &dyn NoteRepository,
) -> Result<Vec<Note>, AppError> {
    note_repository.get_all_notes_feed().await.map_err(|_| {
        AppError::InternalServerError(
            "An internal server error occurred getting the notes feed".to_string(),
        )
    })
}

pub(crate) async fn get_own_notes_feed(
    note_repository: &dyn NoteRepository,
    claims: &Claims,
) -> Result<Vec<Note>, AppError> {
    note_repository
        .get_notes_feed(&claims.sub)
        .await
        .map_err(|_| {
            AppError::InternalServerError(
                "An internal server error occurred getting the notes feed".to_string(),
            )
        })
}

pub(crate) async fn get_note_by_id_as_admin(
    note_repository: &dyn NoteRepository,
    note_id: i64,
) -> Result<Option<Note>, AppError> {
    get_note_by_id(note_repository, note_id).await
}

pub(crate) async fn get_own_note_by_id(
    note_repository: &dyn NoteRepository,
    note_id: i64,
    claims: &Claims,
) -> Result<Option<Note>, AppError> {
    let owner = get_note_owner(note_repository, &note_id).await?;

    if owner != claims.sub {
        return Err(AppError::Forbidden);
    }

    get_note_by_id(note_repository, note_id).await
}

async fn get_note_by_id(
    note_repository: &dyn NoteRepository,
    note_id: i64,
) -> Result<Option<Note>, AppError> {
    note_repository.get_note_by_id(&note_id).await.map_err(|_| {
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
        return Err(AppError::Forbidden);
    }

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
        .map_err(|_| {
            AppError::InternalServerError(
                "An internal server error occurred updating note".to_string(),
            )
        })
}

pub(crate) async fn delete_note_as_admin(
    note_repository: &dyn NoteRepository,
    note_id: i64,
) -> Result<(), AppError> {
    delete_note(note_repository, note_id).await
}

pub(crate) async fn delete_own_note(
    note_repository: &dyn NoteRepository,
    note_id: i64,
    claims: &Claims,
) -> Result<(), AppError> {
    let note_owner = get_note_owner(note_repository, &note_id).await?;

    if note_owner != claims.sub {
        return Err(AppError::Forbidden);
    }

    delete_note(note_repository, note_id).await
}

async fn delete_note(note_repository: &dyn NoteRepository, note_id: i64) -> Result<(), AppError> {
    let rows_deleted = note_repository
        .delete_note(&note_id)
        .await
        .map_err(|_| utils::get_deletion_error())?;

    if rows_deleted == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub(crate) async fn delete_all_notes_self(
    note_repository: &dyn NoteRepository,
    claims: &Claims,
) -> Result<usize, AppError> {
    delete_all_notes_of_user(note_repository, claims.sub.clone()).await
}

pub(crate) async fn delete_all_notes_as_admin(
    note_repository: &dyn NoteRepository,
    user_id: String,
) -> Result<usize, AppError> {
    delete_all_notes_of_user(note_repository, user_id).await
}

async fn delete_all_notes_of_user(
    note_repository: &dyn NoteRepository,
    user_id: String,
) -> Result<usize, AppError> {
    let result = note_repository
        .delete_all_notes_of_user(&user_id)
        .await
        .map_err(|_| utils::get_deletion_error());
    Ok(result?)
}
