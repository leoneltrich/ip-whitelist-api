pub mod implementation;
pub mod interface;

use std::sync::Arc;
use crate::persistence::repository::interface::notes::NoteRepository;

#[derive(Clone)]
pub struct Repositories {
    pub note: Arc<dyn NoteRepository>,
}

impl Repositories {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            note: Arc::new(
                implementation::note::SqliteNoteRepository::new(
                    pool,
                ),
            ),
        }
    }
}
