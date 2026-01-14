use super::FirewallBackend;
use async_trait::async_trait;
use shared::errors::AppError;
use std::net::IpAddr;
use std::time::Duration;

pub struct MockFirewall;

#[async_trait]
impl FirewallBackend for MockFirewall {
    async fn grant_access(
        &self,
        ip: IpAddr,
        port: u16,
        duration: Duration,
    ) -> Result<(), AppError> {
        println!(
            "🔒 [MOCK FIREWALL] Allowing IP: {} on PORT: {} for {:?}",
            ip, port, duration
        );
        Ok(())
    }

    async fn validate_config(&self) -> Result<(), AppError> {
        println!("🔒 [MOCK FIREWALL] Configuration OK");
        Ok(())
    }

    async fn setup(&self) -> Result<(), AppError> {
        println!("Mock setup");
        Ok(())
    }
}
