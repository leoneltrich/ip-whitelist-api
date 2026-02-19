use crate::models::database::refresh_token::RefreshToken;
use crate::models::database::user::User;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::persistence::repository::interface::user::UserRepository;
use crate::security::hashing;
use rand::RngExt;
use shared::auth::jwt;
use shared::auth::models::Claims;
use shared::errors::app_errors::AppError;
use sqlx::Error;
use tracing::{debug, error};

const THIRTY_DAYS_IN_SECONDS: i64 = 60 * 60 * 24 * 30;

pub(crate) fn hash_refresh_token(token: &str) -> String {
    hashing::create_sha256_hash(token)
}

pub fn create_access_token(private_key_pem: &String, user: &User) -> Result<String, AppError> {
    let claims = Claims::new(user.username.clone(), user.is_admin);

    let access_token = jwt::sign(claims, private_key_pem)?;

    debug!("Successfully generated access token");
    Ok(access_token)
}

pub async fn create_refresh_token(
    username: &str,
    refresh_token_repository: &dyn RefreshTokenRepository,
) -> Result<String, AppError> {
    let refresh_token = generate_plain_token();

    save_refresh_token(&refresh_token, username, refresh_token_repository)
        .await
        .map_err(|e| {
            error!("An error occurred saving the refresh token: {}", e);
            AppError::InternalServerError(
                "An error occurred during the refresh token generation".to_string(),
            )
        })?;

    debug!("Successfully generated refresh token");
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

    debug!("Finished preparing refresh token, saving...");
    refresh_token_repository
        .create_refresh_token(&refresh_token)
        .await
}

fn generate_plain_token() -> String {
    let random_bytes: [u8; 32] = rand::rng().random();
    let plain_token = hex::encode(random_bytes);
    plain_token
}

pub async fn get_user_optional(
    repository: &dyn UserRepository,
    username: &String,
) -> Result<Option<User>, AppError> {
    let user_option = repository.get_user_by_name(username).await.map_err(|e| {
        error!("An error occurred accessing the database: {}", e);
        AppError::InternalServerError("An internal server error occurred".to_string())
    })?;
    Ok(user_option)
}
