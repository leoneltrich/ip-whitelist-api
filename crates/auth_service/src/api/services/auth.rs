use crate::state::AppState;
use rand::Rng;
use shared::auth::models::{Claims, LoginRequest, LoginResponse};
use shared::errors::AppError;
use sqlx::Error;

use crate::models::database::refresh_token::RefreshToken;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::security::hashing;
use shared::auth::jwt;

const THIRTY_DAYS_IN_SECONDS: i64 = 60 * 60 * 24 * 30;

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

    let is_valid_hash = hashing::verify_password(&req.password, hash_to_verify).unwrap_or(false);

    if let (true, Some(user)) = (is_valid_hash, user_found) {
        let claims = Claims::new(user.username.clone(), user.is_admin);

        let access_token = jwt::sign(claims, &state.config.private_key_pem)?;
        let refresh_token = create_refresh_token(&user.username, &*state.repositories.refresh_token).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
        })
    } else {
        Err(AppError::InvalidCredentials)
    }
}

async fn create_refresh_token(
    username: &str,
    refresh_token_repository: &dyn RefreshTokenRepository,
) -> Result<String, AppError> {
    let refresh_token = generate_plain_token();
    save_refresh_token(&refresh_token, username, refresh_token_repository)
        .await
        .map_err(|_| {
            AppError::InternalServerError(
                "An error occurred during the refresh token generation".to_string(),
            )
        })?;
    Ok(refresh_token)
}

async fn save_refresh_token(
    plain_token: &str,
    username: &str,
    refresh_token_repository: &dyn RefreshTokenRepository,
) -> Result<usize, Error> {
    let hashed_token = hashing::create_sha256_hash(&plain_token);
    let current_time = chrono::Utc::now().timestamp();
    let expires_at = current_time + THIRTY_DAYS_IN_SECONDS;

    let refresh_token = RefreshToken {
        token_hash: hashed_token.to_string(),
        username: username.to_string(),
        expires_at: expires_at,
        created_at: current_time,
        is_revoked: false,
    };

    refresh_token_repository
        .create_refresh_token(&refresh_token)
        .await
}

fn generate_plain_token() -> String {
    let random_bytes: [u8; 32] = rand::rng().random();
    let plain_token = hex::encode(random_bytes);
    plain_token
}
