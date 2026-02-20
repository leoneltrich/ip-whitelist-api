use super::FirewallBackend;
use async_trait::async_trait;
use shared::errors::app_errors::AppError;
use std::net::IpAddr;
use std::time::Duration;
use tracing::info;

pub struct MockFirewall;

#[async_trait]
impl FirewallBackend for MockFirewall {
    async fn setup(&self) -> Result<(), AppError> {
        info!("Mock firewall setup");
        Ok(())
    }

    async fn grant_access(
        &self,
        ip: IpAddr,
        port: u16,
        duration: Duration,
    ) -> Result<(), AppError> {
        info!(
            "[MOCK FIREWALL] Granting access to IP: {} on PORT: {} for {:?}",
            ip, port, duration
        );
        Ok(())
    }

    async fn validate_config(&self) -> Result<(), AppError> {
        info!("[MOCK FIREWALL] Configuration OK");
        Ok(())
    }
}
