use async_trait::async_trait;
use sqlx::{Error, SqlitePool};
use crate::models::database::note::Note;
use crate::persistence::repository::interface::notes::NoteRepository;

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
    async fn create_note(&self, note: &Note) -> Result<usize, Error> {
        todo!()
    }

    async fn get_note_by_id(&self, note_id: &str) -> Result<Option<Note>, Error> {
        todo!()
    }

    async fn update_note(&self, note: &Note) -> Result<usize, Error> {
        todo!()
    }

    async fn delete_note(&self, note_id: &str) -> Result<usize, Error> {
        todo!()
    }

    async fn get_all_notes(&self) -> Result<Vec<Note>, Error> {
        todo!()
    }

    async fn delete_all_notes_of_user(&self, user_id: &str) -> Result<usize, Error> {
        todo!()
    }
}