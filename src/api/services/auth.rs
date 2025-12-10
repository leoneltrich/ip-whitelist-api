// src/api/services/auth.rs

// 1. Imports from your pluralized modules
use crate::models::api::auth::{Claims, LoginRequest, LoginResponse};
use crate::errors::AppError;
use crate::state::AppState;

// 2. Security imports
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};

pub async fn login(state: &AppState, req: LoginRequest) -> Result<LoginResponse, AppError> {
    // 1. Fetch User
    let user_option = state.repositories.user.get_user(&req.username).await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let user = match user_option {
        Some(u) => u,
        None => return Err(AppError::InvalidCredentials),
    };

    // 2. Verify Password
    let is_valid = verify(req.password, &user.password_hash)
        .map_err(|e| AppError::InternalServerError(format!("Verification failed: {}", e)))?;

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