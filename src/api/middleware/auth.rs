// src/api/middleware/auth.rs
use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};
use axum::extract::State;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::errors::AppError;
use crate::models::api::auth::Claims;
use crate::state::AppState;

// A hardcoded token for demonstration
const AUTH_TOKEN: &str = "my-secret-token";

pub async fn auth(
    // Axum injects the state here automatically
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Get the 'Authorization' header
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::InvalidToken)?;

    // 2. Parse "Bearer <token>"
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::InvalidToken);
    }
    let token = &auth_header[7..]; // Strip "Bearer " prefix

    // 3. Decode and Verify JWT
    // We create a DecodingKey from the secret in our Config
    let secret_bytes = state.config.jwt_secret.as_bytes();
    let decoding_key = DecodingKey::from_secret(secret_bytes);

    // Validation::default() checks signature AND expiration (exp) automatically
    let claim_data = decode::<Claims>(
        token,
        &decoding_key,
        &Validation::default(),
    )
        .map_err(|_| AppError::InvalidToken)?; // If expired or invalid, return 401

    // 4. (Optional but recommended) Inject the user info into the request
    // This allows downstream handlers to know WHO is making the request.
    // request.extensions_mut().insert(claim_data.claims);

    // 5. Token is valid, proceed
    let response = next.run(request).await;
    Ok(response)
}