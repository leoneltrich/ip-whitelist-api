use crate::models::api::server::{CreateServerRequest, ServerResponse, UpdateServerRequest};
use crate::models::database::server::Server;
use crate::persistence::repository::Repositories;
use shared::errors::app_errors::AppError;
use sqlx::error::ErrorKind;

// --- CREATE ---
pub async fn create_server(repos: &Repositories, req: CreateServerRequest) -> Result<(), AppError> {
    let server = Server {
        servername: req.servername.clone(),
        port: req.port,
        api_startup_method: req.api_startup_method,
        api_startup_link: req.api_startup_link,
        api_startup_token: req.api_startup_token,
    };

    match repos.server.create_server(&server).await {
        Ok(_) => Ok(()),
        Err(e) => {
            // Check for Unique Constraint Violation (Duplicate Servername)
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == ErrorKind::UniqueViolation {
                    return Err(AppError::Conflict(format!(
                        "Server '{}' already exists",
                        req.servername
                    )));
                }
            }
            Err(AppError::InternalServerError(e.to_string()))
        }
    }
}

// --- LIST ---
pub async fn list_servers(repos: &Repositories) -> Result<Vec<ServerResponse>, AppError> {
    let servers = repos
        .server
        .list_all_servers()
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let response = servers
        .into_iter()
        .map(|s| ServerResponse {
            servername: s.servername,
            port: s.port,
            api_startup_method: s.api_startup_method,
            api_startup_link: s.api_startup_link,
            has_token: s.api_startup_token.is_some(),
        })
        .collect();

    Ok(response)
}

// --- GET ONE ---
pub async fn get_server(repos: &Repositories, name: String) -> Result<ServerResponse, AppError> {
    let server = repos
        .server
        .get_server_by_name(&name)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or(AppError::NotFound)?;

    Ok(ServerResponse {
        servername: server.servername,
        port: server.port,
        api_startup_method: server.api_startup_method,
        api_startup_link: server.api_startup_link,
        has_token: server.api_startup_token.is_some(),
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
        api_startup_method: req.api_startup_method,
        api_startup_link: req.api_startup_link,
        api_startup_token: req.api_startup_token,
    };

    let rows = repos
        .server
        .update_server(&current_name, &server)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.kind() == ErrorKind::UniqueViolation {
                    return AppError::Conflict(format!(
                        "Server name '{}' is already taken",
                        req.servername
                    ));
                }
            }
            AppError::InternalServerError(e.to_string())
        })?;

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// --- DELETE ---
pub async fn delete_server(repos: &Repositories, name: String) -> Result<(), AppError> {
    let rows = repos
        .server
        .delete_server(&name)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
