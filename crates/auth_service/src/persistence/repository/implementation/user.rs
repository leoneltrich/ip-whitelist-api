// src/persistence/repository/implementation/user

use crate::models::database::user::User;
use crate::persistence::repository::interface::user::UserRepository;
use async_trait::async_trait;
use sqlx::{Error, SqlitePool};

pub struct SqliteUserRepository {
    pub pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, user: &User) -> Result<usize, Error> {
        let result =
            sqlx::query("INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, ?)")
                .bind(&user.username)
                .bind(&user.password_hash)
                .bind(user.is_admin)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_user_by_name(&self, username: &str) -> Result<Option<User>, Error> {
        let result = sqlx::query_as::<_, User>(
            "SELECT username, password_hash, is_admin FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update_user(&self, user: &User) -> Result<usize, Error> {
        let result =
            sqlx::query("UPDATE users SET password_hash = ?, is_admin = ? WHERE username = ?")
                .bind(&user.password_hash)
                .bind(user.is_admin)
                .bind(&user.username)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn delete_user(&self, username: &str) -> Result<usize, Error> {
        let result = sqlx::query("DELETE FROM users WHERE username = ?")
            .bind(username)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let result =
            sqlx::query_as::<_, User>("SELECT username, password_hash, is_admin FROM users")
                .fetch_all(&self.pool)
                .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::initialization::initialize;

    async fn setup_db() -> SqlitePool {
        let pool = initialize(":memory:").await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        let user = User {
            username: "testuser".to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        };

        let rows = repo.create_user(&user).await.unwrap();
        assert_eq!(rows, 1);
    }

    #[tokio::test]
    async fn test_create_user_duplicate_fails() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        let user = User {
            username: "testuser".to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        };

        repo.create_user(&user).await.unwrap();
        let result = repo.create_user(&user).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_by_name_found() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        let username = "testuser";
        repo.create_user(&User {
            username: username.to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        }).await.unwrap();

        let result = repo.get_user_by_name(username).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().username, username);
    }

    #[tokio::test]
    async fn test_get_user_by_name_not_found() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);

        let result = repo.get_user_by_name("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        let username = "testuser";
        repo.create_user(&User {
            username: username.to_string(),
            password_hash: "old_hash".to_string(),
            is_admin: false,
        }).await.unwrap();

        let updated_user = User {
            username: username.to_string(),
            password_hash: "new_hash".to_string(),
            is_admin: true,
        };

        let rows = repo.update_user(&updated_user).await.unwrap();
        assert_eq!(rows, 1);

        let fetched = repo.get_user_by_name(username).await.unwrap().unwrap();
        assert_eq!(fetched.password_hash, "new_hash");
        assert!(fetched.is_admin);
    }

    #[tokio::test]
    async fn test_update_user_not_found_returns_zero_rows() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        let user = User {
            username: "nonexistent".to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        };

        let rows = repo.update_user(&user).await.unwrap();
        assert_eq!(rows, 0);
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        let username = "testuser";
        repo.create_user(&User {
            username: username.to_string(),
            password_hash: "hash".to_string(),
            is_admin: false,
        }).await.unwrap();

        let rows = repo.delete_user(username).await.unwrap();
        assert_eq!(rows, 1);

        let result = repo.get_user_by_name(username).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_delete_user_not_found_returns_zero_rows() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);

        let rows = repo.delete_user("nonexistent").await.unwrap();
        assert_eq!(rows, 0);
    }

    #[tokio::test]
    async fn test_get_all_users_empty() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);

        let users = repo.get_all_users().await.unwrap();
        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn test_get_all_users_multiple() {
        let pool = setup_db().await;
        let repo = SqliteUserRepository::new(pool);
        repo.create_user(&User { username: "u1".to_string(), password_hash: "h1".to_string(), is_admin: false }).await.unwrap();
        repo.create_user(&User { username: "u2".to_string(), password_hash: "h2".to_string(), is_admin: true }).await.unwrap();

        let users = repo.get_all_users().await.unwrap();
        assert_eq!(users.len(), 2);
    }
}
