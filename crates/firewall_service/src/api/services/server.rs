use crate::models::api::server::{CreateServerRequest, ServerResponse, UpdateServerRequest};
use crate::models::database::server::Server;
use crate::persistence::repository::Repositories;
use shared::errors::app_errors::AppError;
use sqlx::error::ErrorKind;
use tracing::{error, info};

// --- CREATE ---
pub async fn create_server(repos: &Repositories, req: CreateServerRequest) -> Result<(), AppError> {
    let server = Server {
        servername: req.servername.clone(),
        port: req.port,
        protocol: req.protocol,
    };

    match repos.server.create_server(&server).await {
        Ok(_) => {
            info!("Server '{}' created successfully.", req.servername);
            Ok(())
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == ErrorKind::UniqueViolation {
                    info!("Server with name '{}' already exists.", req.servername);
                    return Err(AppError::Conflict(format!(
                        "Server '{}' already exists",
                        req.servername
                    )));
                }
            }
            error!("Failed to create server in database: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}

// --- LIST ---
pub async fn list_servers(repos: &Repositories) -> Result<Vec<ServerResponse>, AppError> {
    let servers = repos.server.list_all_servers().await.map_err(|e| {
        error!("Failed to list servers from database: {}", e);
        AppError::InternalServerError
    })?;

    let response: Vec<ServerResponse> = servers
        .into_iter()
        .map(|s| ServerResponse {
            servername: s.servername,
            port: s.port,
        })
        .collect();

    info!("Returning {} servers", response.len());
    Ok(response)
}

// --- GET ONE ---
pub async fn get_server(repos: &Repositories, name: String) -> Result<ServerResponse, AppError> {
    let server = repos
        .server
        .get_server_by_name(&name)
        .await
        .map_err(|e| {
            error!("Failed to get server from database: {}", e);
            AppError::InternalServerError
        })?
        .ok_or_else(|| {
            info!("No server found with name '{}'.", name);
            AppError::NotFound
        })?;

    info!("Returning server '{}'", server.servername);

    Ok(ServerResponse {
        servername: server.servername,
        port: server.port,
    })
}

// --- UPDATE ---
pub async fn update_server(
    repos: &Repositories,
    current_name: String,
    req: UpdateServerRequest,
) -> Result<(), AppError> {
    let server = Server {
        servername: req.servername.clone(),
        port: req.port,
        protocol: req.protocol,
    };

    let rows = repos
        .server
        .update_server(&current_name, &server)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == ErrorKind::UniqueViolation {
                    info!("Server with name '{}' already exists.", req.servername);
                    return AppError::Conflict(format!(
                        "Server name '{}' is already taken",
                        req.servername
                    ));
                }
            }
            error!("Failed to update server in database: {}", e);
            AppError::InternalServerError
        })?;

    if rows == 0 {
        info!("No server found with name '{}'.", current_name);
        return Err(AppError::NotFound);
    }

    info!("Server '{}' updated successfully.", req.servername);
    Ok(())
}

// --- DELETE ---
pub async fn delete_server(repos: &Repositories, name: String) -> Result<(), AppError> {
    let rows = repos.server.delete_server(&name).await.map_err(|e| {
        error!("Failed to delete server from database: {}", e);
        AppError::InternalServerError
    })?;

    if rows == 0 {
        info!("No server found with name '{}'.", name);
        return Err(AppError::NotFound);
    }

    info!("Server '{}' deleted successfully.", name);
    Ok(())
}
