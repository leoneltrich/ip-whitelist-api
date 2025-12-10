// src/api/services/auth.rs

use crate::errors::AppError;
// 1. Imports from your pluralized modules
use crate::models::api::auth::{Claims, LoginRequest, LoginResponse};
use crate::state::AppState;

use crate::security::hashing;
use jsonwebtoken::{EncodingKey, Header, encode};

pub async fn login(state: &AppState, req: LoginRequest) -> Result<LoginResponse, AppError> {
    // 1. Fetch User
    let user_option = state
        .repositories
        .user
        .get_user(&req.username)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let user = match user_option {
        Some(u) => u,
        None => return Err(AppError::InvalidCredentials),
    };

    // 2. Verify Password
    let is_valid = hashing::verify_password(&req.password, &user.password_hash)
        .map_err(|_| AppError::InternalServerError("Verification failed".to_string()))?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // 3. Create Claims WITH is_admin flag from the DB entity
    let claims = Claims::new(user.username, user.is_admin);

    // 4. Encode Token
    let secret_bytes = state.config.jwt_secret.as_bytes();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_bytes),
    )
    .map_err(|e| AppError::InternalServerError(format!("Token signing failed: {}", e)))?;

    Ok(LoginResponse { token })
}
