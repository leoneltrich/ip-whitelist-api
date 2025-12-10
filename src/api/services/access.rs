use std::net::IpAddr;
use std::time::Duration;
use crate::state::AppState;
use crate::models::api::access::{AccessRequest, AccessResponse};
use crate::errors::AppError;

const ACCESS_DURATION: Duration = Duration::from_secs(12 * 60 * 60);

pub async fn grant_access(
    state: &AppState,
    _req: AccessRequest,
    requester_ip: IpAddr
) -> Result<AccessResponse, AppError> {

    state.firewall.grant_access(requester_ip, ACCESS_DURATION).await?;

    Ok(AccessResponse {
        status: "success".to_string(),
        message: format!("IP {} has been whitelisted.", requester_ip),
        expires_in: "12h".to_string(),
    })
}