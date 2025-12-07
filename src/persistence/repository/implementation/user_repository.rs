// src/persistence/repository/implementation/user_repository.rs

use crate::models::database::user::User;
use crate::persistence::repository::interface::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteUserRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, user: &User) -> Result<usize, String> {
        let result = sqlx::query(
            "INSERT INTO users (username, password_hash) VALUES (?, ?)"
        )
            .bind(&user.username)
            .bind(&user.password_hash)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_user(&self, username: &str) -> Result<Option<User>, String> {
        // query_as maps the database row directly to your Struct!
        // Note: Your User struct needs `#[derive(sqlx::FromRow)]` for this magic to work.
        let result = sqlx::query_as::<_, User>(
            "SELECT username, password_hash FROM users WHERE username = ?"
        )
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn update_user(&self, user: &User) -> Result<usize, String> {
        let result = sqlx::query(
            "UPDATE users SET password_hash = ? WHERE username = ?"
        )
            .bind(&user.password_hash)
            .bind(&user.username)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() as usize)
    }

    async fn delete_user(&self, username: &str) -> Result<usize, String> {
        let result = sqlx::query("DELETE FROM users WHERE username = ?")
            .bind(username)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() as usize)
    }
}