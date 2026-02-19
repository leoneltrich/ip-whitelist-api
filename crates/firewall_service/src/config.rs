use std::env;
use std::fs;
use shared::logging::models::LogConfig;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub public_key_pem: String,
    pub firewall_backend: String,
    pub database_path: String,
    pub log_config: LogConfig
}

impl AppConfig {
    pub fn new() -> Self {
        let public_key_path = env::var("PUBLIC_KEY_PATH").expect("PUBLIC_KEY_PATH must be set");

        Self {
            public_key_pem: fs::read_to_string(&public_key_path)
                .unwrap_or_else(|_| panic!("Failed to read public key from {}", public_key_path)),
            firewall_backend: env::var("FIREWALL_BACKEND").expect("FIREWALL_BACKEND must be set"),
            database_path: env::var("DATABASE_PATH").expect("DATABASE_PATH must be set"),
            log_config: LogConfig::from_env(),
        }
    }
}
