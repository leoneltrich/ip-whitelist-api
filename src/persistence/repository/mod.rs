// src/persistence/repository/mod.rs
pub mod interface;
pub mod implementation;

use std::sync::Arc;
use sqlx::SqlitePool;
use crate::persistence::repository::implementation::server::SqliteServerRepository;
use crate::persistence::repository::implementation::whitelist::SqliteWhitelistRepository;
use crate::persistence::repository::interface::server::ServerRepository;
use crate::persistence::repository::interface::whitelist::WhitelistRepository;
// Import the concrete type ONLY here, inside the persistence boundary
use self::implementation::user::SqliteUserRepository;
// Import the interface
use self::interface::user::UserRepository;

// This container holds your Interfaces. 
// Main will only ever see this struct.
#[derive(Clone)]
pub struct Repositories {
    pub user: Arc<dyn UserRepository + Send + Sync>,
    pub server: Arc<dyn ServerRepository + Send + Sync>,
    pub whitelist: Arc<dyn WhitelistRepository + Send + Sync>,
}

impl Repositories {
    /// This is the "Factory" that hides the concrete types from the rest of the app.
    pub fn new(pool: SqlitePool) -> Self {
        let user_repo = SqliteUserRepository { pool: pool.clone() }; // Clone pool is cheap
        let server_repo = SqliteServerRepository { pool: pool.clone() };
        let whitelist_repo = SqliteWhitelistRepository { pool: pool.clone() };

        Self {
            user: Arc::new(user_repo),
            server: Arc::new(server_repo),
            whitelist: Arc::new(whitelist_repo)
        }
    }
}