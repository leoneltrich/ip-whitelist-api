use std::env;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub jwt_secret: String,
    // Add other env vars here (e.g., database_url, redis_host)
}

impl AppConfig {
    pub fn new() -> Self {
        // In a real app, consider using the 'config' crate for more robustness,
        // but for now, simple env var loading is fine.
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}