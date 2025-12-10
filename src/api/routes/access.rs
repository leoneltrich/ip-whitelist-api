use axum::{
    extract::{ConnectInfo, State},
    Json,
    http::HeaderMap,
    response::IntoResponse,
};
use std::net::{IpAddr, SocketAddr};
use crate::state::AppState;
use crate::models::api::access::{AccessRequest, AccessResponse};
use crate::api::services::access as access_service;
use crate::errors::AppError;

#[utoipa::path(
    post,
    path = "/users/access",
    request_body = AccessRequest,
    responses(
        (status = 200, description = "Access granted", body = AccessResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Firewall backend error")
    ),
    security(
        ("jwt" = [])
    )
)]
pub async fn request_access(
    State(state): State<AppState>,
    headers: HeaderMap,
    // Axum extracts the socket info automatically
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<AccessRequest>,
) -> Result<impl IntoResponse, AppError> {

    // 1. Determine the Real IP
    // Priority: X-Forwarded-For Header -> Direct Connection
    let ip = get_real_ip(&headers, addr).unwrap_or(addr.ip());

    // 2. Call Service
    let response = access_service::grant_access(&state, payload, ip).await?;

    Ok(Json(response))
}

/// Helper to parse X-Forwarded-For or fall back to socket address
fn get_real_ip(headers: &HeaderMap, addr: SocketAddr) -> Option<IpAddr> {
    headers
        .get("X-Forwarded-For")
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.split(',').next()) // Get first IP in list
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
        .or(Some(addr.ip()))
}