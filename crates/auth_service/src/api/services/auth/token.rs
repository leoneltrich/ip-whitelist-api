use crate::api::services::auth::utils::{create_access_token, create_refresh_token, get_user_optional, hash_refresh_token};
use crate::models::api::auth::TokenRefreshResponse;
use crate::models::database::refresh_token::RefreshToken;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::persistence::repository::interface::user::UserRepository;
use shared::errors::AppError;

async fn refresh(
    refresh_token: &str,
    token_repository: &dyn RefreshTokenRepository,
    user_repository: &dyn UserRepository,
    private_key_pem: &String,
    username: &String,
) -> Result<TokenRefreshResponse, AppError> {
    let refresh_token_hash = hash_refresh_token(refresh_token);

    let stored_refresh_token =
        get_stored_refresh_token(token_repository, &refresh_token_hash).await?;

    if let Some(value) = validate_refresh_token(stored_refresh_token, username) {
        return value;
    }

    revoke_refresh_token(token_repository, &refresh_token_hash).await?;

    let user = match get_user_optional(user_repository, username).await? {
        Some(user) => user,
        None => {
            return Err(AppError::InternalServerError(
                "An internal server error occurred".to_string(),
            ));
        }
    };

    let access_token = create_access_token(private_key_pem, &user)?;
    let refresh_token = create_refresh_token(username, token_repository).await?;

    let response = TokenRefreshResponse {
        access_token,
        refresh_token,
    };

    Ok(response)
}

async fn revoke_refresh_token(token_repository: &dyn RefreshTokenRepository, refresh_token_hash: &String) -> Result<(), AppError> {
    token_repository
        .revoke_refresh_token(&refresh_token_hash)
        .await
        .map_err(|_| {
            AppError::InternalServerError(
                "An internal server error occurred revoking the original refresh token".to_string(),
            )
        })?;
    Ok(())
}

async fn get_stored_refresh_token(
    repository: &dyn RefreshTokenRepository,
    refresh_token_hash: &String,
) -> Result<RefreshToken, AppError> {
    let stored_refresh_token = match repository
        .get_refresh_token(&refresh_token_hash)
        .await
        .map_err(|_| {
            AppError::InternalServerError(
                "An internal server error occurred validating the refresh token".to_string(),
            )
        })? {
        Some(token) => token,
        None => return Err(AppError::InvalidRefreshToken),
    };
    Ok(stored_refresh_token)
}

fn validate_refresh_token(
    stored_refresh_token: RefreshToken,
    username: &str,
) -> Option<Result<TokenRefreshResponse, AppError>> {
    let current_time = chrono::Utc::now().timestamp();

    if stored_refresh_token.is_revoked {
        return Some(Err(AppError::InvalidRefreshToken));
    }

    if stored_refresh_token.expires_at < current_time {
        return Some(Err(AppError::InvalidRefreshToken));
    }

    if stored_refresh_token.username != username {
        return Some(Err(AppError::InvalidRefreshToken));
    }

    None
}
