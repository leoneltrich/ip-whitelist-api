use crate::models::database::server::Server;
use crate::persistence::repository::interface::server::ServerRepository;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SqliteServerRepository {
    pub pool: SqlitePool,
}

#[async_trait]
impl ServerRepository for SqliteServerRepository {
    async fn create_server(&self, server: &Server) -> Result<usize, String> {
        let result = sqlx::query(
            "INSERT INTO servers (
                servername, port,
                api_startup_method, api_startup_link, api_startup_token
            ) VALUES (?, ?, ?, ?, ?)"
        )
            .bind(&server.servername)
            .bind(server.port)
            .bind(&server.api_startup_method)
            .bind(&server.api_startup_link)
            .bind(&server.api_startup_token)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_server_by_name(&self, name: &str) -> Result<Option<Server>, String> {
        let result = sqlx::query_as::<_, Server>(
            "SELECT * FROM servers WHERE servername = ?"
        )
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn list_all_servers(&self) -> Result<Vec<Server>, String> {
        let result = sqlx::query_as::<_, Server>("SELECT * FROM servers")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn delete_server(&self, name: &str) -> Result<usize, String> {
        let result = sqlx::query("DELETE FROM servers WHERE servername = ?")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(result.rows_affected() as usize)
    }
}