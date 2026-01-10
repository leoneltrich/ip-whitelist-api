// src/api/services/auth.rs

use shared::errors::AppError;
use shared::auth_models::{Claims, LoginRequest, LoginResponse};
use crate::state::AppState;

use crate::security::hashing;
use shared::jwt;


pub async fn login(state: &AppState, req: LoginRequest) -> Result<LoginResponse, AppError> {

    let user_option = state
        .repositories
        .user
        .get_user_by_name(&req.username)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let dummy_hash = "$2b$12$J5YHkgw7QJrhL8etGOZMpObtChFL4rxSDdYNAMqC.k5AWikbDkhau";

    let (hash_to_verify, user_found) = match &user_option {
        Some(user) => (user.password_hash.as_str(), Some(user)),
        None => (dummy_hash, None),
    };

    let is_valid_hash = hashing::verify_password(&req.password, hash_to_verify)
        .unwrap_or(false);

    if let (true, Some(user)) = (is_valid_hash, user_found) {

        let claims = Claims::new(user.username.clone(), user.is_admin);

        let token = jwt::sign(claims, &state.config.private_key_pem)?;

        Ok(LoginResponse { token })

    } else {
        Err(AppError::InvalidCredentials)
    }
}
