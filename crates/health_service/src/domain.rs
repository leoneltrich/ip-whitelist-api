use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HealthStatus {
    Up,
    Down,
    Degraded,
    Starting,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealth {
    pub name: String,
    pub status: HealthStatus,
    pub url: String,
    pub latency_ms: Option<u64>,
    pub last_checked: DateTime<Utc>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub services: HashMap<String, ServiceHealth>,
    pub last_updated: DateTime<Utc>,
}

impl SystemHealth {
    pub fn new() -> Self {
        Self {
            status: HealthStatus::Starting,
            services: HashMap::new(),
            last_updated: Utc::now(),
        }
    }
}
