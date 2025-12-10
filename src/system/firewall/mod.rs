use async_trait::async_trait;
use std::net::IpAddr;
use std::time::Duration;
use crate::errors::AppError;

// Re-export implementations
pub mod mock;
pub mod nftables;

#[async_trait]
pub trait FirewallBackend: Send + Sync {
    /// Grants access to a specific IP for a specific duration.
    /// If the IP exists, the timer is reset.
    async fn grant_access(&self, ip: IpAddr, duration: Duration) -> Result<(), AppError>;

    /// (Optional) Validates that the firewall is configured correctly on startup
    async fn validate_config(&self) -> Result<(), AppError>;
}