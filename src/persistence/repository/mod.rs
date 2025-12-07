// src/persistence/repository/mod.rs
pub mod interface;
pub mod implementation;

use std::sync::Arc;
use sqlx::SqlitePool;

// Import the concrete type ONLY here, inside the persistence boundary
use self::implementation::user_repository::SqliteUserRepository;
// Import the interface
use self::interface::user_repository::UserRepository;

// This container holds your Interfaces. 
// Main will only ever see this struct.
#[derive(Clone)]
pub struct Repositories {
    pub user: Arc<dyn UserRepository + Send + Sync>,
    // Later you will add: pub server: Arc<dyn ServerRepository...>,
}

impl Repositories {
    /// This is the "Factory" that hides the concrete types from the rest of the app.
    pub fn new(pool: SqlitePool) -> Self {
        let user_repo = SqliteUserRepository { pool: pool.clone() }; // Clone pool is cheap

        Self {
            user: Arc::new(user_repo),
        }
    }
}