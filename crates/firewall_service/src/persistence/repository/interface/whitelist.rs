use crate::models::database::whitelist::WhitelistEntry;
use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait WhitelistRepository: Send + Sync {
    async fn add_entry(&self, entry: &WhitelistEntry) -> Result<(), Error>;

    // We can only update expiration, because changing the other fields
    // would mean changing the Primary Key (which requires delete + insert).
    async fn update_expiration(&self, entry: &WhitelistEntry) -> Result<(), Error>;

    async fn remove_entry(&self, server: &str, user: &str, ip: &str) -> Result<(), Error>;

    async fn get_entry(
        &self,
        server: &str,
        user: &str,
        ip: &str,
    ) -> Result<Option<WhitelistEntry>, Error>;
}
