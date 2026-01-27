#[derive(Debug, sqlx::FromRow)]
pub(crate) struct RefreshToken {
    pub(crate) token_hash: String,
    pub(crate) username: i64,
    pub(crate) expires_at: i64,
    pub(crate) created_at: i64,
    pub(crate) is_revoked: bool,
}