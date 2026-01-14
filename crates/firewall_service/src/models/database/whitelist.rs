use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct WhitelistEntry {
    pub servername: String, // Foreign Key to Servers
    pub username: String,   // Foreign Key to Users
    pub ip_address: String,
    pub expiration: i64, // Unix Timestamp (seconds)
}
