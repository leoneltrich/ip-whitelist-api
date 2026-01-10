pub mod interface;
pub mod implementation;

use crate::persistence::repository::interface::user::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub user: Arc<dyn UserRepository>,
}

impl Repositories {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            user: Arc::new(crate::persistence::repository::implementation::user::SqliteUserRepository::new(pool)),
        }
    }
}
