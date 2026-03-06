use shared::logging::models::LogConfig;
use std::env;
use std::fs;
use tracing::info;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub public_key_pem: String,
    pub firewall_backend: String,
    pub database_path: String,
    pub log_config: LogConfig,
    pub listen_port: u16,
    pub bind_address: String,
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
            listen_port: env::var("LISTEN_PORT")
                .ok()
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or_else(|| {
                    info!("No listen port set, defaulting to 3000");
                    return 3000;
                }),
            bind_address: env::var("BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .trim()
                .to_string()
                .replace(" ", ""),
        }
    }
}
