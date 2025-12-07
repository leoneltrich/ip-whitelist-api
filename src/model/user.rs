#[derive(Debug)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password_hash: String,
}