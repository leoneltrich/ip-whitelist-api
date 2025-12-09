use std::env;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub jwt_secret: String,
    // Add other env vars here (e.g., database_url, redis_host)
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}