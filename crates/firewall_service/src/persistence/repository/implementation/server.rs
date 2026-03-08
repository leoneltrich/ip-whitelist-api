use crate::models::database::server::Server;
use crate::persistence::repository::interface::server::ServerRepository;
use async_trait::async_trait;
use sqlx::{Error, SqlitePool};

pub struct SqliteServerRepository {
    pub pool: SqlitePool,
}

impl SqliteServerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ServerRepository for SqliteServerRepository {
    async fn create_server(&self, server: &Server) -> Result<usize, Error> {
        let result =
            sqlx::query("INSERT INTO servers (servername, port, protocol) VALUES (?, ?, ?)")
                .bind(&server.servername)
                .bind(server.port)
                .bind(&server.protocol)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn get_server_by_name(&self, name: &str) -> Result<Option<Server>, Error> {
        let result = sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE servername = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    async fn list_all_servers(&self) -> Result<Vec<Server>, Error> {
        let result = sqlx::query_as::<_, Server>("SELECT * FROM servers")
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    async fn delete_server(&self, name: &str) -> Result<usize, Error> {
        let result = sqlx::query("DELETE FROM servers WHERE servername = ?")
            .bind(name)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() as usize)
    }

    async fn update_server(&self, current_name: &str, server: &Server) -> Result<usize, Error> {
        let result = sqlx::query(
            "UPDATE servers SET
                servername = ?,
                port = ?,
                protocol = ?
            WHERE servername = ?",
        )
        .bind(&server.servername)
        .bind(server.port)
            .bind(&server.protocol)
        .bind(current_name) // Identifying the row to update
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() as usize)
    }
}
