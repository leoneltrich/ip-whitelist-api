use async_trait::async_trait;
use std::net::IpAddr;
use std::time::Duration;
use shared::errors::AppError;

// Re-export implementations
pub mod mock;
pub mod nftables;

#[async_trait]
pub trait FirewallBackend: Send + Sync {
    async fn setup(&self) -> Result<(), AppError>;

    async fn grant_access(&self, ip: IpAddr, port: u16, duration: Duration) -> Result<(), AppError>;

    async fn validate_config(&self) -> Result<(), AppError>;
}