// src/persistence/repository/interface/user_repository.rs

use rusqlite::Result;
use crate::model::user::User; // <-- Import from top-level model

pub trait UserRepository {
    fn get_user_by_name(&self, username: &str) -> Result<Option<User>>;
}