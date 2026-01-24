use crate::models::database::note::{NewNote, Note, UpdateNote};
use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait NoteRepository: Send + Sync {
    /// Inserts a new note.
    /// Returns sqlx error or the new entries id
    async fn create_note(&self, note: &NewNote) -> Result<i64, Error>;

    /// Returns sqlx error or an optional containing the retrieved note or nothing
    async fn get_note_by_id(&self, note_id: &str) -> Result<Option<Note>, Error>;

    /// Returns sqlx error or the owner id of the note
    async fn get_note_owner_id(&self, note_id: &i64) -> Result<Option<String>, Error>;

    /// Updates the note.
    /// Returns sqlx error or the number of rows affected
    async fn update_note(&self, note: &UpdateNote) -> Result<usize, Error>;

    /// Deletes the note.
    /// Returns sqlx error or the number of rows affected
    async fn delete_note(&self, note_id: &i64) -> Result<usize, Error>;

    /// Returns an sqlx error or a vector of all notes that are owned by the user or public
    /// in descending order
    async fn get_notes_feed(&self, user_id: &str) -> Result<Vec<Note>, Error>;

    /// Deletes all notes of user
    /// Returns sqlx error or the number of rows affected
    async fn delete_all_notes_of_user(&self, user_id: &str) -> Result<usize, Error>;
}
