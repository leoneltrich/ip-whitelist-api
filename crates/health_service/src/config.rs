use serde::Deserialize;
use std::env;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,

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

fn default_host() -> String {
    env::var("HEALTH_HOST").unwrap_or_else(|_| "0.0.0.0".to_string())
}

fn default_port() -> u16 {
    env::var("HEALTH_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3002)
}

fn default_refresh_interval() -> u64 {
    10000
}
fn default_required() -> bool {
    true
}
fn default_timeout() -> u64 {
    1000
}
fn default_initial_delay() -> u64 {
    5
}

impl Config {
    pub async fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path =
            env::var("HEALTH_CONFIG_PATH").unwrap_or_else(|_| "services.json".to_string());

        let host_override = env::var("HEALTH_HOST").ok();
        let port_override = env::var("HEALTH_PORT").ok().and_then(|p| p.parse().ok());

        Self::load_internal(&config_path, host_override, port_override).await
    }

    pub async fn load_internal(
        path: &str,
        host_override: Option<String>,
        port_override: Option<u16>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Err(format!("Configuration file not found at: {}", path).into());
        }

        let content = fs::read_to_string(path).await?;
        let mut config: Config = serde_json::from_str(&content)?;

        if let Some(h) = host_override {
            config.host = h;
        }

        if let Some(p) = port_override {
            config.port = p;
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_load_returns_error_when_file_not_found() {
        // Arrange
        let path = "non_existent_file.json";

        // Act
        let result = Config::load_internal(path, None, None).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_parses_valid_json_with_defaults() {
        // Arrange
        let mut temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{
            "services": [
                {
                    "name": "test_service",
                    "url": "http://localhost"
                }
            ]
        }"#;
        writeln!(temp_file, "{}", json_content).unwrap();

        let path = temp_file.path().to_str().unwrap();

        // Act
        let config = Config::load_internal(path, None, None)
            .await
            .expect("Failed to load state");

        // Assert
        assert_eq!(config.host, "0.0.0.0"); // Default host
        assert_eq!(config.port, 3002); // Default port
        assert_eq!(config.refresh_interval_ms, 10000); // Default interval
        assert_eq!(config.services.len(), 1);

        let service = &config.services[0];
        assert_eq!(service.name, "test_service");
    }

    #[tokio::test]
    async fn test_load_applies_port_override() {
        // Arrange
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", r#"{ "services": [] }"#).unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Act
        let config = Config::load_internal(path, None, Some(9090)).await.unwrap();

        // Assert
        assert_eq!(config.port, 9090);
    }

    #[tokio::test]
    async fn test_load_applies_host_override() {
        // Arrange
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", r#"{ "services": [] }"#).unwrap();
        let path = temp_file.path().to_str().unwrap();

        // Act
        let config = Config::load_internal(path, Some("127.0.0.1".to_string()), None).await.unwrap();

        // Assert
        assert_eq!(config.host, "127.0.0.1");
    }
}
