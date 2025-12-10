use crate::models::database::server::Server;
use async_trait::async_trait;

#[async_trait]
pub trait ServerRepository {
    async fn create_server(&self, server: &Server) -> Result<usize, String>;
    async fn get_server_by_name(&self, name: &str) -> Result<Option<Server>, String>;
    async fn list_all_servers(&self) -> Result<Vec<Server>, String>;
    async fn delete_server(&self, name: &str) -> Result<usize, String>;
}