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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialization::initialize;
    use crate::persistence::repository::implementation::user::SqliteUserRepository;
    use crate::persistence::repository::interface::user::UserRepository;
    use crate::models::database::user::User;

    async fn setup_db() -> SqlitePool {
        initialize(":memory:").await.unwrap()
    }

    async fn create_test_user(pool: &SqlitePool, username: &str) {
        let user_repo = SqliteUserRepository::new(pool.clone());
        let user = User {
            username: username.to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        };
        user_repo.create_user(&user).await.unwrap();
    }

    #[tokio::test]
    async fn test_create_refresh_token_success() {
        let pool = setup_db().await;
        let repo = SqliteRefreshTokenRepository::new(pool.clone());
        create_test_user(&pool, "testuser").await;

        let token = RefreshToken {
            token_hash: "tokenhash".to_string(),
            username: "testuser".to_string(),
            expires_at: 1000,
            created_at: 500,
            is_revoked: false,
        };

        let rows = repo.create_refresh_token(&token).await.unwrap();
        assert_eq!(rows, 1);
    }

    #[tokio::test]
    async fn test_get_refresh_token_found() {
        let pool = setup_db().await;
        let repo = SqliteRefreshTokenRepository::new(pool.clone());
        create_test_user(&pool, "testuser").await;
        let token_hash = "tokenhash";
        repo.create_refresh_token(&RefreshToken {
            token_hash: token_hash.to_string(),
            username: "testuser".to_string(),
            expires_at: 1000,
            created_at: 500,
            is_revoked: false,
        }).await.unwrap();

        let result = repo.get_refresh_token(token_hash).await.unwrap().unwrap();
        assert_eq!(result.username, "testuser");
    }

    #[tokio::test]
    async fn test_get_refresh_token_not_found() {
        let pool = setup_db().await;
        let repo = SqliteRefreshTokenRepository::new(pool);

        let result = repo.get_refresh_token("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_revoke_refresh_token_success() {
        let pool = setup_db().await;
        let repo = SqliteRefreshTokenRepository::new(pool.clone());
        create_test_user(&pool, "testuser").await;
        let token_hash = "tokenhash";
        repo.create_refresh_token(&RefreshToken {
            token_hash: token_hash.to_string(),
            username: "testuser".to_string(),
            expires_at: 1000,
            created_at: 500,
            is_revoked: false,
        }).await.unwrap();

        let rows = repo.revoke_refresh_token(token_hash).await.unwrap();
        assert_eq!(rows, 1);

        let revoked = repo.get_refresh_token(token_hash).await.unwrap().unwrap();
        assert!(revoked.is_revoked);
    }

    #[tokio::test]
    async fn test_revoke_all_refresh_tokens_of_user() {
        let pool = setup_db().await;
        let repo = SqliteRefreshTokenRepository::new(pool.clone());
        create_test_user(&pool, "testuser").await;

        repo.create_refresh_token(&RefreshToken {
            token_hash: "t1".to_string(),
            username: "testuser".to_string(),
            expires_at: 1000,
            created_at: 500,
            is_revoked: false,
        }).await.unwrap();
        repo.create_refresh_token(&RefreshToken {
            token_hash: "t2".to_string(),
            username: "testuser".to_string(),
            expires_at: 1000,
            created_at: 500,
            is_revoked: false,
        }).await.unwrap();

        let rows = repo.revoke_all_refresh_tokens_of_user("testuser").await.unwrap();
        assert_eq!(rows, 2);
        
        assert!(repo.get_refresh_token("t1").await.unwrap().unwrap().is_revoked);
        assert!(repo.get_refresh_token("t2").await.unwrap().unwrap().is_revoked);
    }

    #[tokio::test]
    async fn test_foreign_key_cascade() {
        let pool = setup_db().await;
        let repo = SqliteRefreshTokenRepository::new(pool.clone());
        let user_repo = SqliteUserRepository::new(pool);

        let user = User {
            username: "delete_me".to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        };
        user_repo.create_user(&user).await.unwrap();

        repo.create_refresh_token(&RefreshToken {
            token_hash: "cascade_token".to_string(),
            username: "delete_me".to_string(),
            expires_at: 1000,
            created_at: 500,
            is_revoked: false,
        }).await.unwrap();

        // Delete user
        user_repo.delete_user("delete_me").await.unwrap();

        // Token should be gone due to ON DELETE CASCADE
        let token = repo.get_refresh_token("cascade_token").await.unwrap();
        assert!(token.is_none());
    }
}
