use crate::models::database::refresh_token::RefreshToken;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use async_trait::async_trait;
use sqlx::{Error, SqlitePool};

pub struct SqliteRefreshTokenRepository {
    pub pool: SqlitePool,
}

impl SqliteRefreshTokenRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RefreshTokenRepository for SqliteRefreshTokenRepository {
    async fn delete_refresh_token(&self, refresh_token_hash: &str) -> Result<usize, Error> {
        let result = sqlx::query("DELETE FROM refresh_tokens WHERE token_hash = ?")
            .bind(&refresh_token_hash)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_refresh_token(
        &self,
        refresh_token_hash: &str,
    ) -> Result<Option<RefreshToken>, Error> {
        let result =
            sqlx::query_as::<_, RefreshToken>("SELECT * FROM refresh_tokens WHERE token_hash = ?")
                .bind(&refresh_token_hash)
                .fetch_optional(&self.pool)
                .await?;
        Ok(result)
    }

    async fn create_refresh_token(&self, refresh_token: &RefreshToken) -> Result<usize, Error> {
        let result = sqlx::query("INSERT INTO refresh_tokens (token_hash, username, expires_at, created_at) VALUES (?, ?, ?, ?)")
            .bind(&refresh_token.token_hash)
            .bind(&refresh_token.username)
            .bind(&refresh_token.expires_at)
            .bind(&refresh_token.created_at)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn update_refresh_token(&self, refresh_token: &RefreshToken) -> Result<usize, Error> {
        let result = sqlx::query("UPDATE refresh_tokens SET username = ?, expires_at = ?, created_at = ?, is_revoked = ? WHERE token_hash = ?")
            .bind(refresh_token.username.to_string())
            .bind(refresh_token.expires_at)
            .bind(refresh_token.created_at)
            .bind(refresh_token.is_revoked)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn revoke_refresh_token(&self, refresh_token_hash: &str) -> Result<usize, Error> {
        let result = sqlx::query("UPDATE refresh_tokens SET is_revoked = 1 WHERE token_hash = ?")
            .bind(&refresh_token_hash)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn revoke_all_refresh_tokens_of_user(&self, username: &str) -> Result<usize, Error> {
        let result = sqlx::query("UPDATE refresh_tokens SET is_revoked = 1 WHERE username = ?")
            .bind(&username)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() as usize)
    }
}
