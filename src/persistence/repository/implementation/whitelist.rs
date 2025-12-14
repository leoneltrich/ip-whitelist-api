use crate::models::database::whitelist::WhitelistEntry;
use crate::persistence::repository::interface::whitelist::WhitelistRepository;
use async_trait::async_trait;
use sqlx::{SqlitePool, Error};

pub struct SqliteWhitelistRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl WhitelistRepository for SqliteWhitelistRepository {
    async fn add_entry(&self, entry: &WhitelistEntry) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO whitelist (servername, username, ip_address, expiration)
             VALUES (?, ?, ?, ?)"
        )
            .bind(&entry.servername)
            .bind(&entry.username)
            .bind(&entry.ip_address)
            .bind(entry.expiration)
            .execute(&self.pool)
            .await?; // No mapping to string, just propagate '?'

        Ok(())
    }

    async fn update_expiration(&self, entry: &WhitelistEntry) -> Result<(), Error> {
        sqlx::query(
            "UPDATE whitelist SET expiration = ?
             WHERE servername = ? AND username = ? AND ip_address = ?"
        )
            .bind(entry.expiration)
            .bind(&entry.servername)
            .bind(&entry.username)
            .bind(&entry.ip_address)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn remove_entry(&self, server: &str, user: &str, ip: &str) -> Result<(), Error> {
        sqlx::query(
            "DELETE FROM whitelist
             WHERE servername = ? AND username = ? AND ip_address = ?"
        )
            .bind(server)
            .bind(user)
            .bind(ip)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_entry(&self, server: &str, user: &str, ip: &str) -> Result<Option<WhitelistEntry>, Error> {
        let result = sqlx::query_as::<_, WhitelistEntry>(
            "SELECT * FROM whitelist
             WHERE servername = ? AND username = ? AND ip_address = ?"
        )
            .bind(server)
            .bind(user)
            .bind(ip)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }
}