use crate::models::database::note::Note;
use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait NoteRepository: Send + Sync {
    /// Inserts a new note. Returns rows affected.
    /// Returns sqlx error or the number of rows affected
    async fn create_note(&self, note: &Note) -> Result<usize, Error>;

    /// returns a note option
    /// Returns sqlx error or an optional containing the retrieved note or nothing
    async fn get_note_by_id(&self, note_id: &str) -> Result<Option<Note>, Error>;

    /// Updates the note.
    /// Returns sqlx error or the number of rows affected
    async fn update_note(&self, note: &Note) -> Result<usize, Error>;

    /// Deletes the note.
    /// Returns sqlx error or the number of rows affected
    async fn delete_note(&self, note_id: &str) -> Result<usize, Error>;

    /// Returns all notes.
    /// Returns an sqlx error or a vector of all notes
    async fn get_all_notes(&self) -> Result<Vec<Note>, Error>;

    /// Deletes all notes of user
    /// Returns sqlx error or the number of rows affected
    async fn delete_all_notes_of_user(&self, user_id: &str) -> Result<usize, Error>;
}
