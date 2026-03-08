use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Server {
    pub servername: String,
    pub port: u16,
    pub protocol: String,
}
