// src/api/middleware/auth.rs
use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};

// A hardcoded token for demonstration
const AUTH_TOKEN: &str = "my-secret-token";

pub async fn auth(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Get the 'Authorization' header
    let token = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.replace("Bearer ", "")); // Remove "Bearer " prefix if present

    // 2. Check if the token matches our hardcoded secret
    match token {
        Some(t) if t == AUTH_TOKEN => {
            // Token is valid, proceed to the next handler
            let response = next.run(request).await;
            Ok(response)
        }
        _ => {
            // Token is missing or invalid, reject request
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}