// src/api/services/auth.rs

// 1. Imports from your pluralized modules
use crate::models::api::auth::{Claims, LoginRequest, LoginResponse};
use crate::errors::AppError;
use crate::state::AppState;

// 2. Security imports
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};

pub async fn login(state: &AppState, req: LoginRequest) -> Result<LoginResponse, AppError> {
    // 1. Attempt to fetch the user
    let user_option = state.repositories.user.get_user(&req.username).await
        .map_err(|e| AppError::InternalServerError(e))?;

    // 2. Check if user exists (Security: Handle generic error)
    let user = match user_option {
        Some(u) => u,
        None => return Err(AppError::InvalidCredentials),
    };

    // 3. Verify the password hash
    let is_valid = verify(req.password, &user.password_hash)
        .map_err(|e| AppError::InternalServerError(format!("Verification failed: {}", e)))?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    let claims = Claims::new(user.username);

    // 4. Encode the Token using the Secret from AppState Config
    // We convert the String secret into bytes with .as_bytes()
    let secret_bytes = state.config.jwt_secret.as_bytes();

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_bytes),
    )
        .map_err(|e| AppError::InternalServerError(format!("Token signing failed: {}", e)))?;

    // --- JWT GENERATION END ---

    Ok(LoginResponse { token })
}