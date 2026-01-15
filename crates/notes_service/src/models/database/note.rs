#[derive(Debug, sqlx::FromRow)]
pub struct Note {
    pub(crate) note_id: i32,
    pub(crate) owner_id: String,
    pub(crate) is_public_read: bool,
    pub(crate) is_public_write: bool,
    pub(crate) title: Option<String>,
    pub(crate) content: String,
}
