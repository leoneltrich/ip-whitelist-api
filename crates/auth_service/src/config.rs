use std::env;
use std::fs;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub private_key_pem: String,
    pub public_key_pem: String,
    pub database_path: String,
}

impl AppConfig {
    pub fn new() -> Self {
        let private_key_path = env::var("PRIVATE_KEY_PATH").expect("PRIVATE_KEY_PATH must be set");
        let public_key_path = env::var("PUBLIC_KEY_PATH").expect("PUBLIC_KEY_PATH must be set");

        Self {
            private_key_pem: fs::read_to_string(&private_key_path)
                .unwrap_or_else(|_| panic!("Failed to read private key from {}", private_key_path)),
            public_key_pem: fs::read_to_string(&public_key_path)
                .unwrap_or_else(|_| panic!("Failed to read public key from {}", public_key_path)),
            database_path: env::var("DATABASE_PATH").expect("DATABASE_PATH must be set"),
        }
    }
}