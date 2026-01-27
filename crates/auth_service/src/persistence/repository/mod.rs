pub mod implementation;
pub mod interface;

use crate::persistence::repository::interface::user::UserRepository;
use std::sync::Arc;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;

#[derive(Clone)]
pub struct Repositories {
    pub user: Arc<dyn UserRepository>,
    pub refresh_token: Arc<dyn RefreshTokenRepository>,
}

impl Repositories {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            user: Arc::new(
                implementation::user::SqliteUserRepository::new(
                    pool.clone(),
                ),
            ),
            refresh_token: Arc::new(
                implementation::refresh_token::SqliteRefreshTokenRepository::new(
                    pool.clone(),
                ),
            )
        }
    }
}
