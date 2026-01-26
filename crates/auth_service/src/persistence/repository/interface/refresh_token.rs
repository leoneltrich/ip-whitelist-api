#[async_trait::async_trait]
pub trait RefreshTokenRepository {
    async fn delete_refresh_token(&self, refresh_token_hash: &str) -> Result<(), sqlx::Error>;
    async fn get_refresh_token(&self, refresh_token_hash: &str) -> Result<Option<String>, sqlx::Error>;
    async fn create_refresh_token(&self, refresh_token: &str) -> Result<(), sqlx::Error>;
    async fn update_refresh_token(&self, refresh_token: &str) -> Result<(), sqlx::Error>;
}