// src/api/services/auth.rs

use crate::errors::AppError;
// 1. Imports from your pluralized modules
use crate::models::api::auth::{Claims, LoginRequest, LoginResponse};
use crate::state::AppState;

use crate::security::hashing;
use jsonwebtoken::{EncodingKey, Header, encode};

use subtle::ConstantTimeEq; // Optional but recommended for boolean checks, though verify_password is the main bottleneck here.

pub async fn login(state: &AppState, req: LoginRequest) -> Result<LoginResponse, AppError> {


    let user_option = state
        .repositories
        .user
        .get_user_by_name(&req.username)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let dummy_hash = "$argon2id$v=19$m=19456,t=2,p=1$DummyHashSaltStringShouldBeValidLength$DummyHashSignatureStringShouldBeValidLength";

    let (hash_to_verify, user_found) = match &user_option {
        Some(user) => (user.password_hash.as_str(), Some(user)),
        None => (dummy_hash, None),
    };

    let is_valid_hash = hashing::verify_password(&req.password, hash_to_verify)
        .unwrap_or(false);

    if let (true, Some(user)) = (is_valid_hash, user_found) {

        let claims = Claims::new(user.username.clone(), user.is_admin);

        let secret_bytes = state.config.jwt_secret.as_bytes();
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret_bytes),
        )
            .map_err(|e| AppError::InternalServerError(format!("Token signing failed: {}", e)))?;

        Ok(LoginResponse { token })

    } else {
        Err(AppError::InvalidCredentials)
    }
}
