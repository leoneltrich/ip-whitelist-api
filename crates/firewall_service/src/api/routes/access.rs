use crate::api::services;
use crate::models::api::access::{AccessRequest, AccessResponse, AccessStatusResponse};
use crate::state::AppState;
use axum::extract::Path;
use axum::{
    extract::{ConnectInfo, State}, http::HeaderMap,
    response::IntoResponse,
    Extension,
    Json,
};
use shared::auth::models::Claims;
use shared::errors::AppError;
use std::net::SocketAddr;
use shared::utils;

#[utoipa::path(
    post,
    path = "/api/v1/users/access",
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
    let ip = utils::get_real_ip(&headers, addr).unwrap_or(addr.ip());

    // 2. Call Service
    let response = services::access::grant_access(&state, req, ip, &claims.sub).await?;

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/access/{server}/status",
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
    let ip = utils::get_real_ip(&headers, addr).ok_or(AppError::InternalServerError(
        "Could not determine IP".into(),
    ))?;

    let response = services::access::get_access_status(&state, server, claims.sub, ip).await?;

    Ok(Json(response))
}

