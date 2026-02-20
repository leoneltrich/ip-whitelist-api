use crate::api::services::auth::utils::{
    create_access_token, create_refresh_token,
};
use crate::models::api::auth::{LoginRequest, LoginResponse, LogoutResponse};
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::persistence::repository::interface::user::UserRepository;
use crate::security::hashing;
use crate::security::hashing::create_sha256_hash;
use shared::errors::app_errors::AppError;
use tracing::{error, info, warn};

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
        info!(
            "Failed login attempt detected with username: {}",
            &req.username
        );
        Err(AppError::InvalidCredentials)
    }
}

pub(crate) async fn logout(
    repository: &dyn RefreshTokenRepository,
    user: &str,
    refresh_token: &str,
) -> Result<LogoutResponse, AppError> {
    let token_hash = create_sha256_hash(refresh_token);

    let stored_token = repository
        .get_refresh_token(&token_hash)
        .await
        .map_err(|e| {
            error!("An error occurred accessing the database: {}", e);
            AppError::InternalServerError
        })?
        .ok_or(AppError::InvalidRefreshToken)?;

    if stored_token.username != user {
        warn!(
            "Suspicious logout attempt detected, user with username: {}, tried to log out refresh token of user, {}",
            &user, &stored_token.username
        );
        return Err(AppError::InvalidRefreshToken);
    }

    repository
        .revoke_refresh_token(&token_hash)
        .await
        .map_err(|e| {
            error!("An error occurred accessing the database: {}", e);
            AppError::InternalServerError
        })?;

    let response = LogoutResponse {
        success: true,
        refresh_token: refresh_token.to_string(),
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::api::auth::LoginRequest;
    use crate::models::database::refresh_token::RefreshToken;
    use crate::models::database::user::User;
    use crate::persistence::repository::interface::refresh_token::MockRefreshTokenRepository;
    use crate::persistence::repository::interface::user::MockUserRepository;
    use crate::security::hashing;
    use rsa::{pkcs8::EncodePrivateKey, RsaPrivateKey};
    use std::sync::OnceLock;

    static TEST_PRIVATE_KEY: OnceLock<String> = OnceLock::new();

    fn get_test_rsa_key() -> &'static String {
        TEST_PRIVATE_KEY.get_or_init(|| {
            let mut rng = rand::rng();
            let bits = 2048;
            let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
            priv_key
                .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
                .expect("failed to encode key")
                .to_string()
        })
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = get_test_rsa_key();

        let username = "testuser".to_string();
        let password = "password123".to_string();
        let password_hash = hashing::hash_password(&password).unwrap();
        // ... rest of the test remains the same

        let user = User {
            username: username.clone(),
            password_hash,
            is_admin: false,
        };

        user_repo
            .expect_get_user_by_name()
            .with(mockall::predicate::eq(username.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(User {
                    username: username.clone(),
                    password_hash: user.password_hash.clone(),
                    is_admin: user.is_admin,
                }))
            });

        token_repo
            .expect_create_refresh_token()
            .times(1)
            .returning(|_| Ok(1));

        let req = LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let result = login(&user_repo, &token_repo, &private_key, req).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let mut user_repo = MockUserRepository::new();
        let token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();

        let username = "testuser".to_string();
        let password_hash = hashing::hash_password("correct_password").unwrap();

        user_repo
            .expect_get_user_by_name()
            .times(1)
            .returning(move |_| {
                Ok(Some(User {
                    username: username.clone(),
                    password_hash: password_hash.clone(),
                    is_admin: false,
                }))
            });

        let req = LoginRequest {
            username: "testuser".to_string(),
            password: "wrong_password".to_string(),
        };

        let result = login(&user_repo, &token_repo, &private_key, req).await;

        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let mut user_repo = MockUserRepository::new();
        let token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();

        user_repo
            .expect_get_user_by_name()
            .times(1)
            .returning(|_| Ok(None));

        let req = LoginRequest {
            username: "nonexistent".to_string(),
            password: "any_password".to_string(),
        };

        let result = login(&user_repo, &token_repo, &private_key, req).await;

        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_db_error() {
        let mut user_repo = MockUserRepository::new();
        let token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();

        user_repo
            .expect_get_user_by_name()
            .times(1)
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let req = LoginRequest {
            username: "testuser".to_string(),
            password: "password".to_string(),
        };

        let result = login(&user_repo, &token_repo, &private_key, req).await;

        assert!(matches!(result, Err(AppError::InternalServerError)));
    }

    #[tokio::test]
    async fn test_logout_success() {
        let mut token_repo = MockRefreshTokenRepository::new();
        let username = "testuser";
        let refresh_token = "some_token";
        let token_hash = create_sha256_hash(refresh_token);

        token_repo
            .expect_get_refresh_token()
            .with(mockall::predicate::eq(token_hash.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: token_hash.clone(),
                    username: "testuser".to_string(),
                    expires_at: 1000,
                    created_at: 500,
                    is_revoked: false,
                }))
            });

        token_repo
            .expect_revoke_refresh_token()
            .times(1)
            .returning(|_| Ok(1));

        let result = logout(&token_repo, username, refresh_token).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        assert_eq!(response.refresh_token, refresh_token);
    }

    #[tokio::test]
    async fn test_logout_db_error() {
        let mut token_repo = MockRefreshTokenRepository::new();
        let username = "testuser";
        let refresh_token = "some_token";
        let token_hash = create_sha256_hash(refresh_token);

        token_repo
            .expect_get_refresh_token()
            .with(mockall::predicate::eq(token_hash.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: token_hash.clone(),
                    username: "testuser".to_string(),
                    expires_at: 1000,
                    created_at: 500,
                    is_revoked: false,
                }))
            });

        token_repo
            .expect_revoke_refresh_token()
            .times(1)
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let result = logout(&token_repo, username, refresh_token).await;

        assert!(matches!(result, Err(AppError::InternalServerError)));
    }

    #[tokio::test]
    async fn test_logout_prevent_idor_attack() {
        let mut token_repo = MockRefreshTokenRepository::new();

        let current_user = "user_a";
        let token_of_b = "token_belonging_to_b";

        // 1. The token exists in DB but belongs to "user_b"
        token_repo.expect_get_refresh_token().returning(move |_| {
            Ok(Some(RefreshToken {
                token_hash: "some_hash".into(),
                username: "user_b".into(),
                expires_at: 1000,
                created_at: 500,
                is_revoked: false,
            }))
        });

        // 2. Verification: revoke_refresh_token should NEVER be called
        token_repo.expect_revoke_refresh_token().times(0);

        let result = logout(&token_repo, current_user, token_of_b).await;

        // 3. Result should be an error
        assert!(matches!(result, Err(AppError::InvalidRefreshToken)));
    }
}
