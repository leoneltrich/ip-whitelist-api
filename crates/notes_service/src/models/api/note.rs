use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::database::note::Note;

#[derive(Deserialize, ToSchema)]
pub struct CreateNoteRequest {
    pub(crate) is_public_read: bool,
    pub(crate) is_public_write: bool,
    pub(crate) title: Option<String>,
    pub(crate) content: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateNoteRequest {
    pub(crate) id: String,
    pub(crate) title: Option<String>,
    pub(crate) content: String,
}

#[derive(Serialize, ToSchema)]
pub struct NoteListResponse {
    pub(crate) status: String,
    pub(crate) data: Vec<Note>
}

#[derive(Serialize, ToSchema)]
pub struct SingleNoteResponse {
    pub(crate) status: String,
    pub(crate) data: Note
}

