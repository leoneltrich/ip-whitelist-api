use crate::models::database::server::Server;
use async_trait::async_trait;
use sqlx::Error;

#[async_trait]
pub trait ServerRepository: Send + Sync {
    async fn create_server(&self, server: &Server) -> Result<usize, Error>;
    async fn get_server_by_name(&self, name: &str) -> Result<Option<Server>, Error>;
    async fn list_all_servers(&self) -> Result<Vec<Server>, Error>;
    async fn delete_server(&self, name: &str) -> Result<usize, Error>;
    async fn update_server(&self, current_name: &str, server: &Server) -> Result<usize, Error>;
}
