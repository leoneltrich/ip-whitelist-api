use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;
use crate::config::Config;
use crate::domain::{SystemHealth, ServiceHealth, HealthStatus};
use chrono::{Utc, DateTime};
use tracing::{info, warn, error, debug};
use reqwest::Client;

pub type SharedHealthState = Arc<RwLock<SystemHealth>>;

pub struct HealthMonitor {
    config: Config,
    state: SharedHealthState,
    client: Client,
    start_time: DateTime<Utc>,
}

impl HealthMonitor {
    pub fn new(config: Config, state: SharedHealthState) -> Self {
        Self {
            config,
            state,
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
            start_time: Utc::now(),
        }
    }

    pub async fn run(&self) {
        let interval_ms = self.config.refresh_interval_ms;
        let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));

        info!("Starting Health Monitor loop with interval: {}ms", interval_ms);

        loop {
            interval.tick().await;
            self.check_all_services().await;
        }
    }

    async fn check_all_services(&self) {
        debug!("Running health checks...");
        let mut global_status = HealthStatus::Up;
        let now = Utc::now();

        for service_cfg in &self.config.services {
            let uptime_s = (now - self.start_time).num_seconds() as u64;
            
            let (status, latency, msg) = if uptime_s < service_cfg.initial_delay_s {
                debug!("Service {} is in grace period ({}s < {}s)", service_cfg.name, uptime_s, service_cfg.initial_delay_s);
                (HealthStatus::Starting, None, Some("Grace period".to_string()))
            } else {
                self.ping_service(&service_cfg.url, service_cfg.timeout_ms).await
            };

            // Update Global Status Logic
            if status == HealthStatus::Down && service_cfg.required {
                global_status = HealthStatus::Down;
            } else if status == HealthStatus::Down && !service_cfg.required {
                if global_status != HealthStatus::Down {
                     global_status = HealthStatus::Degraded;
                }
            }

            // Update Individual Service State
            let mut state_guard = self.state.write().await;
            state_guard.services.insert(service_cfg.name.clone(), ServiceHealth {
                name: service_cfg.name.clone(),
                status,
                url: service_cfg.url.clone(),
                latency_ms: latency,
                last_checked: Utc::now(),
                message: msg,
            });
            state_guard.last_updated = Utc::now();
            state_guard.status = global_status;
        }
    }

    async fn ping_service(&self, url: &str, timeout_ms: u64) -> (HealthStatus, Option<u64>, Option<String>) {
        let start = std::time::Instant::now();
        
        let result = self.client.get(url)
            .timeout(Duration::from_millis(timeout_ms))
            .send()
            .await;

        let latency = start.elapsed().as_millis() as u64;

        match result {
            Ok(resp) => {
                if resp.status().is_success() {
                    (HealthStatus::Up, Some(latency), None)
                } else {
                    (HealthStatus::Down, Some(latency), Some(format!("HTTP {}", resp.status())))
                }
            }
            Err(e) => {
                warn!("Health check failed for {}: {}", url, e);
                (HealthStatus::Down, None, Some(e.to_string()))
            }
        }
    }
}
