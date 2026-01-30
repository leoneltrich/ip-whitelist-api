use crate::api::services::auth::utils::{
    create_access_token, create_refresh_token, hash_refresh_token,
};
use crate::models::api::auth::{LoginRequest, LoginResponse, LogoutResponse};
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::persistence::repository::interface::user::UserRepository;
use crate::security::hashing;
use shared::errors::AppError;

pub(crate) mod token;
mod utils;

pub async fn login(
    user_repository: &dyn UserRepository,
    refresh_token_repository: &dyn RefreshTokenRepository,
    private_key_pem: &String,
    req: LoginRequest,
) -> Result<LoginResponse, AppError> {
    let user_option = utils::get_user_optional(user_repository, &req.username).await?;

    let dummy_hash = "$2b$12$J5YHkgw7QJrhL8etGOZMpObtChFL4rxSDdYNAMqC.k5AWikbDkhau";

    let (hash_to_verify, user_found) = match &user_option {
        Some(user) => (user.password_hash.as_str(), Some(user)),
        None => (dummy_hash, None),
    };

    let is_valid_hash = hashing::verify_password(&req.password, hash_to_verify).unwrap_or(false);

    if let (true, Some(user)) = (is_valid_hash, user_found) {
        let access_token = create_access_token(private_key_pem, user)?;
        let refresh_token = create_refresh_token(&user.username, refresh_token_repository).await?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
        })
    } else {
        Err(AppError::InvalidCredentials)
    }
}

pub(crate) async fn logout(
    repository: &dyn RefreshTokenRepository,
    user: &str,
    refresh_token: &str,
) -> Result<LogoutResponse, AppError> {
    let token_hash = hash_refresh_token(refresh_token);
    repository
        .revoke_refresh_token(&token_hash)
        .await
        .map_err(|_| {
            AppError::InternalServerError(
                "An error occurred during the refresh token revocation".to_string(),
            )
        })?;

    let response = LogoutResponse {
        success: true,
        refresh_token: refresh_token.to_string(),
    };

    Ok(response)
}
