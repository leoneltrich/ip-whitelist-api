#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password_hash: String,
    pub(crate) is_admin: bool,
}