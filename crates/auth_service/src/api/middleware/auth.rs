// src/api/middleware/auth.rs
use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response, Extension};
use axum::extract::State;
use shared::errors::AppError;
use shared::auth_models::Claims;
use crate::state::AppState;
use shared::jwt;


pub async fn auth(
    // Axum injects the state here automatically
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::InvalidToken)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::InvalidToken);
    }

    let token = &auth_header[7..]; // Strip "Bearer " prefix

    let claims = jwt::verify(token, &state.config.public_key_pem)
        .map_err(|_| AppError::InvalidToken)?;

    request.extensions_mut().insert(claims);

    let response = next.run(request).await;
    Ok(response)
}

pub async fn require_admin(
    // Extract claims that were inserted by 'auth' above
    Extension(claims): Extension<Claims>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {

    if !claims.is_admin {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(request).await)
}