pub mod implementation;
pub mod interface;

use crate::persistence::repository::interface::server::ServerRepository;
use crate::persistence::repository::interface::whitelist::WhitelistRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub server: Arc<dyn ServerRepository>,
    pub whitelist: Arc<dyn WhitelistRepository>,
}

impl Repositories {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            server: Arc::new(crate::persistence::repository::implementation::server::SqliteServerRepository::new(pool.clone())),
            whitelist: Arc::new(crate::persistence::repository::implementation::whitelist::SqliteWhitelistRepository::new(pool)),
        }
    }
}
