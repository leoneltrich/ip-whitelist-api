use std::net::IpAddr;
use std::time::Duration;
use crate::state::AppState;
use crate::models::api::access::{AccessRequest, AccessResponse};
use crate::errors::AppError;

const ACCESS_DURATION: Duration = Duration::from_secs(12 * 60 * 60);

pub async fn grant_access(
    state: &AppState,
    req: AccessRequest,
    requester_ip: IpAddr
) -> Result<AccessResponse, AppError> {

    let server = state.repositories.server.get_server_by_name(&req.server_id).await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or(AppError::NotFound)?;

    state.firewall.grant_access(requester_ip, server.port, ACCESS_DURATION).await?;

    Ok(AccessResponse {
        status: "success".to_string(),
        message: format!("Access granted to '{}' on port {} for 12h.", server.servername, server.port),
        expires_in: "12h".to_string(),
    })
}