use serde::Deserialize;
use std::env;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_ms: u64,
    
    pub services: Vec<ServiceConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub url: String,
    #[serde(default = "default_required")]
    pub required: bool,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    #[serde(default = "default_initial_delay")]
    pub initial_delay_s: u64,
}

fn default_port() -> u16 {
    env::var("HEALTH_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3002)
}

fn default_refresh_interval() -> u64 { 10000 }
fn default_required() -> bool { true }
fn default_timeout() -> u64 { 1000 }
fn default_initial_delay() -> u64 { 5 }

impl Config {
    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = env::var("HEALTH_CONFIG_PATH").unwrap_or_else(|_| "services.json".to_string());
        
        if !Path::new(&config_path).exists() {
            return Err(format!("Configuration file not found at: {}", config_path).into());
        }

        let content = fs::read_to_string(&config_path).await?;
        let mut config: Config = serde_json::from_str(&content)?;

        // Override port from env if present (double check to ensure env var takes precedence if not set in JSON)
        if let Ok(port_str) = env::var("HEALTH_PORT") {
            if let Ok(p) = port_str.parse() {
                config.port = p;
            }
        }

        Ok(config)
    }
}
