use std::net::IpAddr;
use std::time::Duration;
use chrono::Utc; // Import Chrono for timestamps
use crate::state::AppState;
use crate::models::api::access::{AccessRequest, AccessResponse, AccessStatusResponse};
use crate::models::database::whitelist::WhitelistEntry; // Import the model
use crate::errors::AppError;

const ACCESS_DURATION_SECS: u64 = 12 * 60 * 60;

// CHANGED: Added 'username' argument
pub async fn grant_access(
    state: &AppState,
    req: AccessRequest,
    requester_ip: IpAddr,
    username: &str
) -> Result<AccessResponse, AppError> {

    // 1. Validate Server Exists
    let server = state.repositories.server.get_server_by_name(&req.server_id).await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or(AppError::NotFound)?;

    let ip_string = requester_ip.to_string();

    // 2. Calculate Expiration (Unix Timestamp)
    let expiration_time = Utc::now().timestamp() + (ACCESS_DURATION_SECS as i64);

    // 3. Check if Entry Exists in DB
    // We use the repository to see if this specific user has accessed this server from this IP
    let existing_entry = state.repositories.whitelist
        .get_entry(&server.servername, username, &ip_string)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // 4. Create the Entry Object
    let entry = WhitelistEntry {
        servername: server.servername.clone(),
        username: username.to_string(),
        ip_address: ip_string.clone(),
        expiration: expiration_time,
    };

    // 5. DB Action: Insert vs Update
    if existing_entry.is_some() {
        // Entry exists -> Just extend the time
        state.repositories.whitelist.update_expiration(&entry)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to update whitelist: {}", e)))?;
    } else {
        // New access -> Create new row
        state.repositories.whitelist.add_entry(&entry)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create whitelist entry: {}", e)))?;
    }

    // 6. Apply Firewall Rule (Actual Whitelisting)
    state.firewall.grant_access(
        requester_ip,
        server.port,
        Duration::from_secs(ACCESS_DURATION_SECS)
    ).await?;

    Ok(AccessResponse {
        status: "success".to_string(),
        message: format!("Access granted to '{}' on port {} for 12h.", server.servername, server.port),
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

    // 1. Query the DB
    let entry = state.repositories.whitelist
        .get_entry(&server_name, &username, &ip_str)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // 2. Process Result
    match entry {
        Some(e) => {
            let now = Utc::now().timestamp();
            let is_active = e.expiration > now;
            let remaining_secs = if is_active { e.expiration - now } else { 0 };

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
        },
        None => Ok(AccessStatusResponse {
            server: server_name,
            ip: ip_str,
            is_active: false,
            expiration: None,
            time_remaining: None,
        }),
    }
}