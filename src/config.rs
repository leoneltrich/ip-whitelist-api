use std::env;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub firewall_backend: String,
    pub database_path: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            firewall_backend: env::var("FIREWALL_BACKEND").expect("FIREWALL_BACKEND must be set"),
            database_path: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}