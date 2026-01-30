use crate::models::database::refresh_token::RefreshToken;
use crate::models::database::user::User;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::security::hashing;
use crate::state::AppState;
use rand::Rng;
use shared::auth::jwt;
use shared::auth::models::Claims;
use shared::errors::AppError;
use sqlx::Error;

const THIRTY_DAYS_IN_SECONDS: i64 = 60 * 60 * 24 * 30;

pub(crate) fn hash_refresh_token(token: &str) -> String {
    hashing::create_sha256_hash(token)
}

pub fn create_access_token(state: &&AppState, user: &User) -> Result<String, AppError> {
    let claims = Claims::new(user.username.clone(), user.is_admin);

    let access_token = jwt::sign(claims, &state.config.private_key_pem)?;
    Ok(access_token)
}

pub async fn create_refresh_token(
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
    let hashed_token = hash_refresh_token(plain_token);
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
