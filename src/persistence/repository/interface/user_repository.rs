// src/persistence/repository/interface/user_repository.rs

use rusqlite::Result;
use crate::model::user::User; // <-- Import from top-level model

pub trait UserRepository {
    fn get_user(&self, username: &str) -> Result<Option<User>>;
    fn update_user(&self, user: &User) -> Result<usize>;
    fn delete_user(&self, username: &str) -> Result<usize>;
    fn create_user(&self, user: &User) -> Result<usize>;
}