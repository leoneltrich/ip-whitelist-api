use crate::models::database::note::{NewNote, Note};
use crate::persistence::repository::interface::notes::NoteRepository;
use async_trait::async_trait;
use sqlx::{Error, Row, SqlitePool};

pub struct SqliteNoteRepository {
    pub pool: SqlitePool,
}

impl SqliteNoteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NoteRepository for SqliteNoteRepository {
    async fn create_note(&self, note: &NewNote) -> Result<i64, Error> {
        let result = sqlx::query(
            "INSERT INTO notes (
                   owner_id,
                   is_public_read,
                   is_public_write,
                   title,
                   content,
                   timestamp_created,
                   timestamp_modified
                ) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&note.owner_id)
        .bind(&note.is_public_read)
        .bind(&note.is_public_write)
        .bind(&note.title)
        .bind(&note.content)
        .bind(&note.timestamp_created)
        .bind(&note.timestamp_modified)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    async fn get_note_by_id(&self, note_id: &str) -> Result<Option<Note>, Error> {
        let note = sqlx::query_as::<_, Note>(
            "SELECT 
                note_id, 
                owner_id, 
                is_public_read, 
                is_public_write, 
                title, 
                content, 
                timestamp_created, 
                timestamp_modified
            FROM notes 
            WHERE note_id = ?",
        )
        .bind(note_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(note)
    }

    async fn get_note_owner_id(&self, note_id: &str) -> Result<Option<String>, Error> {
        let row = sqlx::query("SELECT owner_id FROM notes WHERE note_id = ?")
            .bind(note_id)
            .fetch_optional(&self.pool)
            .await?;

        let owner_id = row.map(|r| r.get("owner_id"));
        Ok(owner_id)
    }

    async fn update_note(&self, note: &Note) -> Result<usize, Error> {
        let result = sqlx::query(
            "UPDATE notes 
            SET 
                is_public_read = ?, 
                is_public_write = ?, 
                title = ?, 
                content = ?, 
                timestamp_modified = ?
            WHERE note_id = ?",
        )
        .bind(&note.is_public_read)
        .bind(&note.is_public_write)
        .bind(&note.title)
        .bind(&note.content)
        .bind(&note.timestamp_modified)
        .bind(&note.note_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn delete_note(&self, note_id: &str) -> Result<usize, Error> {
        let result = sqlx::query("DELETE FROM notes WHERE note_id = ?")
            .bind(note_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_notes_feed(&self, user_id: &str) -> Result<Vec<Note>, Error> {
        let notes = sqlx::query_as::<_, Note>(
            "SELECT 
                note_id, 
                owner_id, 
                is_public_read, 
                is_public_write, 
                title, 
                content, 
                timestamp_created, 
                timestamp_modified
            FROM notes
            WHERE owner_id = ? OR is_public_read = 1
            ORDER BY timestamp_modified DESC",
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(notes)
    }

    async fn delete_all_notes_of_user(&self, user_id: &str) -> Result<usize, Error> {
        let result = sqlx::query("DELETE FROM notes WHERE owner_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }
}
