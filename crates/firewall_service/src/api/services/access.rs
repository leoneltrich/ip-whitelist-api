use crate::models::api::access::{AccessRequest, AccessResponse, AccessStatusResponse};
use crate::models::database::whitelist::WhitelistEntry;
use crate::state::AppState;
use chrono::Utc;
use shared::errors::app_errors::AppError;
use std::net::IpAddr;
use std::time::Duration;
use tracing::{debug, error, info};

const ACCESS_DURATION_SECS: u64 = 12 * 60 * 60;

pub async fn grant_access(
    state: &AppState,
    req: AccessRequest,
    requester_ip: IpAddr,
    username: &str,
) -> Result<AccessResponse, AppError> {
    let server = state
        .repositories
        .server
        .get_server_by_name(&req.server_id)
        .await
        .map_err(|e| {
            error!(
                "Failed to get server with name: {} from database: {}",
                &req.server_id, e
            );
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?
        .ok_or_else(|| {
            error!("No server found with name '{}'.", &req.server_id);
            AppError::NotFound
        })?;

    let ip_string = requester_ip.to_string();

    let expiration_time = Utc::now().timestamp() + (ACCESS_DURATION_SECS as i64);

    let existing_entry = state
        .repositories
        .whitelist
        .get_entry(&server.servername, username, &ip_string)
        .await
        .map_err(|e| {
            error!("Failed to get whitelist entry: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?;

    let entry = WhitelistEntry {
        servername: server.servername.clone(),
        username: username.to_string(),
        ip_address: ip_string.clone(),
        expiration: expiration_time,
    };

    if existing_entry.is_some() {
        info!("Updating existing whitelist entry");
        state
            .repositories
            .whitelist
            .update_expiration(&entry)
            .await
            .map_err(|e| {
                error!("Failed to update whitelist entry: {}", e);
                AppError::InternalServerError("An internal server error occurred".to_string())
            })?;
    } else {
        info!("Creating new whitelist entry");
        state
            .repositories
            .whitelist
            .add_entry(&entry)
            .await
            .map_err(|e| {
                error!("Failed to create whitelist entry: {}", e);
                AppError::InternalServerError("An internal server error occurred".to_string())
            })?;
    }

    state
        .firewall
        .grant_access(
            requester_ip,
            server.port,
            Duration::from_secs(ACCESS_DURATION_SECS),
        )
        .await?;

    info!("Access granted to {} on port {}", ip_string, server.port);

    Ok(AccessResponse {
        status: "success".to_string(),
        message: format!(
            "Access granted to '{}' on port {} for 12h.",
            server.servername, server.port
        ),
        expires_in: "12h".to_string(),
    })
}

pub async fn get_access_status(
    state: &AppState,
    server_name: String,
    username: String,
    ip: IpAddr,
) -> Result<AccessStatusResponse, AppError> {
    let ip_str = ip.to_string();

    let entry = state
        .repositories
        .whitelist
        .get_entry(&server_name, &username, &ip_str)
        .await
        .map_err(|e| {
            error!("Failed to get whitelist entry from database: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?;

    match entry {
        Some(e) => {
            let now = Utc::now().timestamp();
            let is_active = e.expiration > now;
            let remaining_secs = if is_active { e.expiration - now } else { 0 };

            if is_active {
                info!(
                    "Active whitelist entry found for user {} on server {}",
                    username, server_name
                );
            } else {
                info!(
                    "Whitelist entry expired for user {} on server {}",
                    username, server_name
                );
            }

            Ok(AccessStatusResponse {
                server: server_name,
                ip: ip_str,
                is_active,
                expiration: Some(e.expiration),
                time_remaining: if is_active {
                    Some(format!("{}m", remaining_secs / 60))
                } else {
                    None
                },
            })
        }
        None => {
            info!(
                "No whitelist entry found for user {} on server {}",
                username, server_name
            );
            Ok(AccessStatusResponse {
                server: server_name,
                ip: ip_str,
                is_active: false,
                expiration: None,
                time_remaining: None,
            })
        }
    }
}
