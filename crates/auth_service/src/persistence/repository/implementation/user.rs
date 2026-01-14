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
        // query_as maps the database row directly to your Struct!
        // Note: Your User struct needs `#[derive(sqlx::FromRow)]` for this magic to work.
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
