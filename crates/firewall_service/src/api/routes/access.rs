use axum::{extract::{ConnectInfo, State}, Json, http::HeaderMap, response::IntoResponse, Extension};
use std::net::{IpAddr, SocketAddr};
use axum::extract::Path;
use crate::api::services;
use crate::state::AppState;
use crate::models::api::access::{AccessRequest, AccessResponse, AccessStatusResponse};
use shared::errors::AppError;
use shared::auth_models::Claims;

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
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(req): Json<AccessRequest>,
) -> Result<impl IntoResponse, AppError> {

    // 1. Determine the Real IP
    // Priority: X-Forwarded-For Header -> Direct Connection
    let ip = get_real_ip(&headers, addr).unwrap_or(addr.ip());

    // 2. Call Service
    let response = services::access::grant_access(&state, req, ip, &claims.sub).await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/users/access/{server}/status",
    params(
        ("server" = String, Path, description = "Server name to check")
    ),
    responses(
        (status = 200, description = "Status retrieved", body = AccessStatusResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(("jwt" = []))
)]
pub async fn check_access_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(server): Path<String>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<AccessStatusResponse>, AppError> {

    let ip = get_real_ip(&headers, addr)
        .ok_or(AppError::InternalServerError("Could not determine IP".into()))?;

    let response = services::access::get_access_status(
        &state,
        server,
        claims.sub,
        ip
    ).await?;

    Ok(Json(response))
}

/// Hardened IP Extraction
///
/// Strictly enforces trusted headers.
fn get_real_ip(headers: &HeaderMap, addr: SocketAddr) -> Option<IpAddr> {

    // The order of those is important to give the CF header a higher priority than the X-Real-IP
    if let Some(ip) = extract_header(headers, "CF-Connecting-IP") {
        return Some(ip);
    }

    // The order of those is important to give the X-Real-IP header a higher priority than the src
    if let Some(ip) = extract_header(headers, "X-Real-IP") {
        return Some(ip);
    }

    Some(addr.ip())
}

/// Helper to extract and parse a single IP header
fn extract_header(headers: &HeaderMap, key: &str) -> Option<IpAddr> {
    headers
        .get(key)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.trim().parse::<IpAddr>().ok())
}