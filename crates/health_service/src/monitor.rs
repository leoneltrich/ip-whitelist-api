use crate::config::Config;
use crate::domain::{HealthStatus, ServiceHealth, SystemHealth};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub type SharedHealthState = Arc<RwLock<SystemHealth>>;

/// Trait to abstract the network probing logic for testing
#[async_trait]
pub trait HealthProber: Send + Sync {
    async fn probe(
        &self,
        url: &str,
        timeout_ms: u64,
    ) -> (HealthStatus, Option<u64>, Option<String>);
}

/// Real implementation using Reqwest
pub struct HttpProber {
    client: Client,
}

impl HttpProber {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }
}

#[async_trait]
impl HealthProber for HttpProber {
    async fn probe(
        &self,
        url: &str,
        timeout_ms: u64,
    ) -> (HealthStatus, Option<u64>, Option<String>) {
        let start = std::time::Instant::now();

        let result = self
            .client
            .get(url)
            .timeout(Duration::from_millis(timeout_ms))
            .send()
            .await;

        let latency = start.elapsed().as_millis() as u64;

        match result {
            Ok(resp) => {
                if resp.status().is_success() {
                    (HealthStatus::Up, Some(latency), None)
                } else {
                    (
                        HealthStatus::Down,
                        Some(latency),
                        Some(format!("HTTP {}", resp.status())),
                    )
                }
            }
            Err(e) => {
                warn!("Health check failed for {}: {}", url, e);
                (HealthStatus::Down, None, Some(e.to_string()))
            }
        }
    }
}

pub struct HealthMonitor {
    config: Config,
    state: SharedHealthState,
    prober: Box<dyn HealthProber>,
    start_time: DateTime<Utc>,
}

impl HealthMonitor {
    pub fn new(config: Config, state: SharedHealthState) -> Self {
        Self {
            config,
            state,
            prober: Box::new(HttpProber::new()),
            start_time: Utc::now(),
        }
    }

    /// Internal constructor for testing with mocks
    pub fn with_prober(
        config: Config,
        state: SharedHealthState,
        prober: Box<dyn HealthProber>,
    ) -> Self {
        Self {
            config,
            state,
            prober,
            start_time: Utc::now(),
        }
    }

    pub async fn run(&self) {
        let interval_ms = self.config.refresh_interval_ms;
        let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));

        info!(
            "Starting Health Monitor loop with interval: {}ms",
            interval_ms
        );

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
                debug!(
                    "Service {} is in grace period ({}s < {}s)",
                    service_cfg.name, uptime_s, service_cfg.initial_delay_s
                );
                (
                    HealthStatus::Starting,
                    None,
                    Some("Grace period".to_string()),
                )
            } else {
                self.prober
                    .probe(&service_cfg.url, service_cfg.timeout_ms)
                    .await
            };

            // Update Global Status Logic
            if status == HealthStatus::Down && service_cfg.required {
                global_status = HealthStatus::Down;
            } else if status == HealthStatus::Down && !service_cfg.required {
                if global_status != HealthStatus::Down {
                    global_status = HealthStatus::Degraded;
                }
            } else if status == HealthStatus::Starting && global_status != HealthStatus::Down {
                // If any service is Starting, and we aren't already Down, we are Starting/Degraded
                // For simplicity, let's treat Starting as a form of "not fully UP" but not critical failure unless required?
                // Actually, if a required service is Starting, the system is technically Starting (not ready).
                if service_cfg.required {
                    global_status = HealthStatus::Starting;
                }
            }

            // Update Individual Service State
            let mut state_guard = self.state.write().await;
            state_guard.services.insert(
                service_cfg.name.clone(),
                ServiceHealth {
                    name: service_cfg.name.clone(),
                    status,
                    url: service_cfg.url.clone(),
                    latency_ms: latency,
                    last_checked: Utc::now(),
                    message: msg,
                },
            );
            state_guard.last_updated = Utc::now();
            state_guard.status = global_status;
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::config::ServiceConfig;
    use std::collections::HashMap;

    struct MockProber {
        responses: HashMap<String, HealthStatus>,
    }

    impl MockProber {
        fn new() -> Self {
            Self {
                responses: HashMap::new(),
            }
        }

        fn set_response(&mut self, url: &str, status: HealthStatus) {
            self.responses.insert(url.to_string(), status);
        }
    }

    #[async_trait]

    impl HealthProber for MockProber {
        async fn probe(
            &self,
            url: &str,
            _timeout_ms: u64,
        ) -> (HealthStatus, Option<u64>, Option<String>) {
            let status = *self.responses.get(url).unwrap_or(&HealthStatus::Down);
            (status, Some(10), None)
        }
    }

    fn create_single_service_config(name: &str, required: bool, initial_delay_s: u64) -> Config {
        Config {
            port: 3000,
            refresh_interval_ms: 1000,
            services: vec![ServiceConfig {
                name: name.to_string(),
                url: format!("http://{}", name),
                required,
                timeout_ms: 100,
                initial_delay_s,
            }],
        }
    }

    #[tokio::test]
    async fn test_check_all_services_returns_starting_status_when_within_grace_period() {

        // Arrange
        let config = create_single_service_config("auth", true, 100); // 100s delay
        let state = Arc::new(RwLock::new(SystemHealth::new()));
        let mut mock = MockProber::new();
        mock.set_response("http://auth", HealthStatus::Up); // Network is actually UP
        let monitor = HealthMonitor::with_prober(config, state.clone(), Box::new(mock));

        // Act
        monitor.check_all_services().await;

        // Assert
        let guard = state.read().await;
        let service = guard.services.get("auth").unwrap();

        // Should be Starting because 0s uptime < 100s delay
        assert_eq!(service.status, HealthStatus::Starting);
        assert_eq!(
            guard.status,
            HealthStatus::Starting,
            "Global status should be Starting if required service is Starting"
        );
    }

    #[tokio::test]

    async fn test_check_all_services_returns_up_status_when_grace_period_is_zero() {

        // Arrange
        let config = create_single_service_config("auth", true, 0); // 0s delay
        let state = Arc::new(RwLock::new(SystemHealth::new()));
        let mut mock = MockProber::new();
        mock.set_response("http://auth", HealthStatus::Up);
        let monitor = HealthMonitor::with_prober(config, state.clone(), Box::new(mock));

        // Act
        monitor.check_all_services().await;

        // Assert
        let guard = state.read().await;
        let service = guard.services.get("auth").unwrap();
        assert_eq!(service.status, HealthStatus::Up);
        assert_eq!(guard.status, HealthStatus::Up);
    }

    #[tokio::test]

    async fn test_check_all_services_sets_global_status_down_when_required_service_is_down() {

        // Arrange
        let config = create_single_service_config("auth", true, 0);
        let state = Arc::new(RwLock::new(SystemHealth::new()));
        let mut mock = MockProber::new();
        mock.set_response("http://auth", HealthStatus::Down);
        let monitor = HealthMonitor::with_prober(config, state.clone(), Box::new(mock));

        // Act
        monitor.check_all_services().await;

        // Assert
        let guard = state.read().await;
        assert_eq!(guard.status, HealthStatus::Down);
    }

    #[tokio::test]

    async fn test_check_all_services_sets_global_status_degraded_when_optional_service_is_down() {

        // Arrange
        let config = create_single_service_config("metrics", false, 0); // Optional
        let state = Arc::new(RwLock::new(SystemHealth::new()));
        let mut mock = MockProber::new();
        mock.set_response("http://metrics", HealthStatus::Down);
        let monitor = HealthMonitor::with_prober(config, state.clone(), Box::new(mock));

        // Act
        monitor.check_all_services().await;

        // Assert
        let guard = state.read().await;
        assert_eq!(guard.status, HealthStatus::Degraded);
    }

    #[tokio::test]

    async fn test_check_all_services_sets_global_status_up_when_all_services_are_up() {

        // Arrange
        let config = Config {
            port: 3000,
            refresh_interval_ms: 1000,
            services: vec![
                ServiceConfig {
                    name: "auth".to_string(),
                    url: "http://auth".to_string(),
                    required: true,
                    timeout_ms: 100,
                    initial_delay_s: 0,
                },
                ServiceConfig {
                    name: "metrics".to_string(),
                    url: "http://metrics".to_string(),
                    required: false,
                    timeout_ms: 100,
                    initial_delay_s: 0,
                },
            ],
        };

        let state = Arc::new(RwLock::new(SystemHealth::new()));
        let mut mock = MockProber::new();
        mock.set_response("http://auth", HealthStatus::Up);
        mock.set_response("http://metrics", HealthStatus::Up);
        let monitor = HealthMonitor::with_prober(config, state.clone(), Box::new(mock));

        // Act
        monitor.check_all_services().await;

        // Assert
        let guard = state.read().await;
        assert_eq!(guard.status, HealthStatus::Up);
    }
}
