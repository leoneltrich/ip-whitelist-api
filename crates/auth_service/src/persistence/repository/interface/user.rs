use crate::models::database::user::User;
use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Inserts a new user. Returns rows affected.
    async fn create_user(&self, user: &User) -> Result<usize, Error>;

    /// returns a user option
    async fn get_user_by_name(&self, username: &str) -> Result<Option<User>, Error>;

    /// Updates the user's password.
    async fn update_user(&self, user: &User) -> Result<usize, Error>;

    /// Deletes the user.
    async fn delete_user(&self, username: &str) -> Result<usize, Error>;

    /// Returns all users.
    async fn get_all_users(&self) -> Result<Vec<User>, Error>;
}
