use crate::model::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    /// Inserts a new user. Returns rows affected.
    async fn create_user(&self, user: &User) -> Result<usize, String>;

    /// returns a user option
    async fn get_user(&self, username: &str) -> Result<Option<User>, String>;

    /// Updates the user's password.
    async fn update_user(&self, user: &User) -> Result<usize, String>;

    /// Deletes the user.
    async fn delete_user(&self, username: &str) -> Result<usize, String>;
}