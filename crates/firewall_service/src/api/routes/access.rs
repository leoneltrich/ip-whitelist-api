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
use shared::errors::app_errors::AppError;
use shared::errors::utoipa_errors::{
    AccessAuthErrorResponse, BadRequestErrorResponse, InternalServerErrorResponse,
    NotFoundErrorResponse, PermissionErrorResponse,
};
use shared::utils;
use std::net::SocketAddr;

#[utoipa::path(
    post,
    path = "/api/v1/users/access",
    request_body = AccessRequest,
    responses(
        (status = 200, description = "Access granted", body = AccessResponse),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    tags = ["Access"],
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
    let ip = utils::get_real_ip(&headers, addr).ok_or(AppError::InternalServerError)?;

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
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    tags = ["Access"],
    security(("jwt" = []))
)]
pub async fn check_access_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(server): Path<String>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<AccessStatusResponse>, AppError> {
    let ip = utils::get_real_ip(&headers, addr).ok_or(AppError::InternalServerError)?;

    let response = services::access::get_access_status(&state, server, claims.sub, ip).await?;

    Ok(Json(response))
}
