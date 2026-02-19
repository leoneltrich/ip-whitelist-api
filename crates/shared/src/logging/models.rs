use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Text,
}

impl FromStr for LogFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(LogFormat::Json),
            "text" => Ok(LogFormat::Text),
            _ => Err(format!("Invalid log format: {}", s)),
        }
    }
}

impl Display for LogFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogFormat::Json => write!(f, "json"),
            LogFormat::Text => write!(f, "text"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogRotation {
    Hourly,
    Daily,
    Never,
}

impl FromStr for LogRotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hourly" => Ok(LogRotation::Hourly),
            "daily" => Ok(LogRotation::Daily),
            "never" => Ok(LogRotation::Never),
            _ => Err(format!("Invalid log rotation: {}", s)),
        }
    }
}

impl Display for LogRotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogRotation::Hourly => write!(f, "hourly"),
            LogRotation::Daily => write!(f, "daily"),
            LogRotation::Never => write!(f, "never"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub rotation: LogRotation,
    pub file_path: Option<String>,
    pub max_files: usize,
}



impl LogConfig {
    pub fn from_env() -> Self {
        Self {
            level: Self::get_value_from_env("LOG_LEVEL", LogLevel::Info),
            format: Self::get_value_from_env("LOG_FORMAT", LogFormat::Json),
            rotation: Self::get_value_from_env("LOG_ROTATION", LogRotation::Daily),
            file_path: env::var("LOG_FILE_PATH").ok(),
            max_files: Self::get_value_from_env("LOG_MAX_FILES", 5),
        }
    }

    fn get_value_from_env<T>(var_name: &str, default: T) -> T
    where
        T: FromStr + Display,
    {
        match env::var(var_name) {
            Ok(value) => value.parse::<T>().unwrap_or_else(|_| {
                println!(
                    "Failed to parse {} from environment variable, using default value: {}",
                    var_name, default
                );
                default
            }),
            Err(_) => {
                println!(
                    "Environment variable {} not set, using default value: {}",
                    var_name, default
                );
                default
            }
        }
    }

    pub fn new(
        level: LogLevel,
        format: LogFormat,
        rotation: LogRotation,
        file_path: Option<String>,
        max_files: usize,
    ) -> Self {
        Self {
            level,
            format,
            rotation,
            file_path,
            max_files,
        }
    }

    pub fn new_dummy() -> Self {
        Self {
            level: LogLevel::Debug,
            format: LogFormat::Text,
            rotation: LogRotation::Never,
            file_path: None,
            max_files: 1,
        }
    }
}
