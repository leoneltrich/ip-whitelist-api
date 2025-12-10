// src/api/middleware/auth.rs
use axum::{extract::Request, http::{StatusCode, HeaderMap}, middleware::Next, response::Response, Extension};
use axum::extract::State;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::errors::AppError;
use crate::models::api::auth::Claims;
use crate::state::AppState;


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
    let secret_bytes = state.config.jwt_secret.as_bytes();
    let decoding_key = DecodingKey::from_secret(secret_bytes);

    let claim_data = decode::<Claims>(
        token,
        &decoding_key,
        &Validation::default(),
    )
        .map_err(|_| AppError::InvalidToken)?; // If expired or invalid, return 401

    request.extensions_mut().insert(claim_data.claims);

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