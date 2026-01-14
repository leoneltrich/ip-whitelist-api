use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Server {
    pub servername: String,
    // ip_address is GONE
    pub port: u16,

    // Wrapped in Option to allow None (NULL in DB)
    pub api_startup_method: Option<String>,
    pub api_startup_link: Option<String>,
    pub api_startup_token: Option<String>,
}
