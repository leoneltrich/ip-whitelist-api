use async_trait::async_trait;
use sqlx::{Error, SqlitePool};
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;

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
    async fn delete_refresh_token(&self, refresh_token_hash: &str) -> Result<(), Error> {
        todo!()
    }

    async fn get_refresh_token(&self, refresh_token_hash: &str) -> Result<Option<String>, Error> {
        todo!()
    }

    async fn create_refresh_token(&self, refresh_token: &str) -> Result<(), Error> {
        todo!()
    }

    async fn update_refresh_token(&self, refresh_token: &str) -> Result<(), Error> {
        todo!()
    }
}