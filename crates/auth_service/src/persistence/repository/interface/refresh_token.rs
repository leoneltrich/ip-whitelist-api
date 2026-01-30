use sqlx::Error;
use crate::models::database::refresh_token::RefreshToken;

#[async_trait::async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn delete_refresh_token(&self, refresh_token_hash: &str) -> Result<usize, Error>;
    async fn get_refresh_token(&self, refresh_token_hash: &str) -> Result<Option<RefreshToken>, Error>;
    async fn create_refresh_token(&self, refresh_token: &RefreshToken) -> Result<usize, Error>;
    async fn update_refresh_token(&self, refresh_token: &RefreshToken) -> Result<usize, Error>;
    async fn revoke_refresh_token(&self, refresh_token_hash: &str) -> Result<usize, Error>;
    async fn revoke_all_refresh_tokens_of_user(&self, username: &str) -> Result<usize, Error>;

}