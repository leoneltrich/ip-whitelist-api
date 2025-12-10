use async_trait::async_trait;
use std::net::IpAddr;
use std::time::Duration;
use crate::errors::AppError;
use super::FirewallBackend;

pub struct MockFirewall;

#[async_trait]
impl FirewallBackend for MockFirewall {
    async fn grant_access(&self, ip: IpAddr, duration: Duration) -> Result<(), AppError> {
        // Log it instead of executing commands
        println!("🔒 [MOCK FIREWALL] Whitelisting IP: {} for {:?}", ip, duration);
        Ok(())
    }

    async fn validate_config(&self) -> Result<(), AppError> {
        println!("🔒 [MOCK FIREWALL] Configuration OK");
        Ok(())
    }
}