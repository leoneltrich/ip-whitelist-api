use crate::api::services::auth::utils::{
    create_access_token, create_refresh_token, get_user_optional,
};
use crate::models::api::auth::TokenRefreshResponse;
use crate::models::database::refresh_token::RefreshToken;
use crate::models::database::user::User;
use crate::persistence::repository::interface::refresh_token::RefreshTokenRepository;
use crate::persistence::repository::interface::user::UserRepository;
use crate::security::hashing::create_sha256_hash;
use shared::errors::app_errors::AppError;
use tracing::{debug, error, info, warn};

pub(crate) async fn refresh(
    refresh_token: &str,
    token_repository: &dyn RefreshTokenRepository,
    user_repository: &dyn UserRepository,
    private_key_pem: &String,
    username: &String,
) -> Result<TokenRefreshResponse, AppError> {
    let refresh_token_hash = create_sha256_hash(refresh_token);

    let stored_refresh_token =
        get_stored_refresh_token(token_repository, &refresh_token_hash).await?;

    validate_refresh_token(stored_refresh_token, username, token_repository).await?;

    revoke_refresh_token(token_repository, &refresh_token_hash).await?;

    let user = get_user(user_repository, username).await?;

    let access_token = create_access_token(private_key_pem, &user)?;
    let refresh_token = create_refresh_token(username, token_repository).await?;
    info!(
        "Issued new refresh and access tokens for user: {}",
        &user.username
    );

    let response = TokenRefreshResponse {
        access_token,
        refresh_token,
    };

    Ok(response)
}

async fn get_user(
    user_repository: &dyn UserRepository,
    username: &String,
) -> Result<User, AppError> {
    let user = match get_user_optional(user_repository, username).await? {
        Some(user) => user,
        None => {
            warn!("User {} not found in database", username);
            return Err(AppError::InternalServerError(
                "An internal server error occurred".to_string(),
            ));
        }
    };
    Ok(user)
}

async fn revoke_refresh_token(
    token_repository: &dyn RefreshTokenRepository,
    refresh_token_hash: &String,
) -> Result<(), AppError> {
    token_repository
        .revoke_refresh_token(&refresh_token_hash)
        .await
        .map_err(|e| {
            error!(
                "Failed to revoke refresh token: {}. Error: {:#?}",
                refresh_token_hash, e
            );
            AppError::InternalServerError("An internal server error occurred".to_string())
        })?;

    debug!("Revoked refresh token with hash: {}", refresh_token_hash);
    Ok(())
}

async fn get_stored_refresh_token(
    repository: &dyn RefreshTokenRepository,
    refresh_token_hash: &String,
) -> Result<RefreshToken, AppError> {
    let stored_refresh_token = match repository
        .get_refresh_token(&refresh_token_hash)
        .await
        .map_err(|e| {
            error!("An error occurred accessing the database: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })? {
        Some(token) => token,
        None => return Err(AppError::InvalidRefreshToken),
    };
    Ok(stored_refresh_token)
}

async fn validate_refresh_token(
    stored_refresh_token: RefreshToken,
    username: &str,
    repo: &dyn RefreshTokenRepository,
) -> Result<(), AppError> {
    let current_time = chrono::Utc::now().timestamp();

    if stored_refresh_token.is_revoked {
        // TODO verify that the log message is not cut off
        warn!(
            "Possible identity theft attempt detected (revoked refresh token reuse): revoked refresh token with hash: {}. Revoking all other refresh tokens of user: {}.",
            &stored_refresh_token.token_hash, &stored_refresh_token.username
        );
        repo.revoke_all_refresh_tokens_of_user(&stored_refresh_token.username)
            .await
            .map_err(|e| {
                error!("An error occurred revoking all refresh tokens: {}", e);
                AppError::InternalServerError("An internal server error occurred".to_string())
            })?;
        return Err(AppError::InvalidRefreshToken);
    }

    if stored_refresh_token.username != username {
        warn!(
            "Possible identity theft attempt detected (stored username doesn't match request): revoked refresh token with hash: {}. Revoking all other refresh tokens of user: {}.",
            &stored_refresh_token.token_hash, &stored_refresh_token.username
        );
        repo.revoke_all_refresh_tokens_of_user(&stored_refresh_token.username)
            .await
            .map_err(|e| {
                error!("An error occurred revoking all refresh tokens: {}", e);
                AppError::InternalServerError("An internal server error occurred".to_string())
            })?;
        return Err(AppError::InvalidRefreshToken);
    }

    if stored_refresh_token.expires_at < current_time {
        info!("Expired refresh token used");
        return Err(AppError::InvalidRefreshToken);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::database::user::User;
    use crate::persistence::repository::interface::refresh_token::MockRefreshTokenRepository;
    use crate::persistence::repository::interface::user::MockUserRepository;
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
    async fn test_refresh_success() {
        let mut user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = get_test_rsa_key();
        // ... rest of the test remains the same

        let username = "testuser".to_string();
        let refresh_token = "valid_token";
        let token_hash = create_sha256_hash(refresh_token);

        let stored_token = RefreshToken {
            token_hash: token_hash.clone(),
            username: username.clone(),
            expires_at: chrono::Utc::now().timestamp() + 3600,
            created_at: chrono::Utc::now().timestamp(),
            is_revoked: false,
        };

        token_repo
            .expect_get_refresh_token()
            .with(mockall::predicate::eq(token_hash.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: stored_token.token_hash.clone(),
                    username: stored_token.username.clone(),
                    expires_at: stored_token.expires_at,
                    created_at: stored_token.created_at,
                    is_revoked: stored_token.is_revoked,
                }))
            });

        token_repo
            .expect_revoke_refresh_token()
            .with(mockall::predicate::eq(token_hash.clone()))
            .times(1)
            .returning(|_| Ok(1));

        let username_clone = username.clone();
        user_repo
            .expect_get_user_by_name()
            .with(mockall::predicate::eq(username.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(User {
                    username: username_clone.clone(),
                    password_hash: "hash".to_string(),
                    is_admin: false,
                }))
            });

        token_repo
            .expect_create_refresh_token()
            .times(1)
            .returning(|_| Ok(1));

        let result = refresh(
            refresh_token,
            &token_repo,
            &user_repo,
            &private_key,
            &username,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
    }

    #[tokio::test]
    async fn test_refresh_revoked_token() {
        let user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();

        let username = "testuser".to_string();
        let refresh_token = "revoked_token";
        let token_hash = create_sha256_hash(refresh_token);

        let username_clone = username.clone();
        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: token_hash.clone(),
                    username: username_clone.clone(),
                    expires_at: chrono::Utc::now().timestamp() + 3600,
                    created_at: chrono::Utc::now().timestamp(),
                    is_revoked: true,
                }))
            });

        token_repo
            .expect_revoke_all_refresh_tokens_of_user()
            .with(mockall::predicate::eq(username.clone()))
            .times(1)
            .returning(|_| Ok(1));

        let result = refresh(
            refresh_token,
            &token_repo,
            &user_repo,
            &private_key,
            &username,
        )
        .await;

        assert!(matches!(result, Err(AppError::InvalidRefreshToken)));
    }

    #[tokio::test]
    async fn test_refresh_expired_token() {
        let user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();

        let username = "testuser".to_string();
        let refresh_token = "expired_token";
        let token_hash = create_sha256_hash(refresh_token);

        let username_clone = username.clone();
        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: token_hash.clone(),
                    username: username_clone.clone(),
                    expires_at: chrono::Utc::now().timestamp() - 3600,
                    created_at: chrono::Utc::now().timestamp() - 7200,
                    is_revoked: false,
                }))
            });

        let result = refresh(
            refresh_token,
            &token_repo,
            &user_repo,
            &private_key,
            &username,
        )
        .await;

        assert!(matches!(result, Err(AppError::InvalidRefreshToken)));
    }

    #[tokio::test]
    async fn test_refresh_identity_theft_attempt() {
        let user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();

        let real_owner = "user_a".to_string();
        let attacker = "user_b".to_string();
        let refresh_token = "token_of_a";
        let token_hash = create_sha256_hash(refresh_token);

        token_repo.expect_get_refresh_token().times(1).returning({
            let h = token_hash.clone();
            let u = real_owner.clone();
            move |_| {
                Ok(Some(RefreshToken {
                    token_hash: h.clone(),
                    username: u.clone(),
                    expires_at: chrono::Utc::now().timestamp() + 3600,
                    created_at: chrono::Utc::now().timestamp(),
                    is_revoked: false,
                }))
            }
        });

        token_repo
            .expect_revoke_all_refresh_tokens_of_user()
            .with(mockall::predicate::eq(real_owner.clone()))
            .times(1)
            .returning(|_| Ok(1));

        let result = refresh(
            refresh_token,
            &token_repo,
            &user_repo,
            &private_key,
            &attacker,
        )
        .await;

        assert!(matches!(result, Err(AppError::InvalidRefreshToken)));
    }

    #[tokio::test]
    async fn test_refresh_db_error_getting_token() {
        let user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();
        let username = "user".to_string();

        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let result = refresh("token", &token_repo, &user_repo, &private_key, &username).await;

        assert!(
            matches!(result, Err(AppError::InternalServerError(msg)) if msg == "An internal server error occurred")
        );
    }

    #[tokio::test]
    async fn test_refresh_db_error_revoking_token() {
        let user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();
        let username = "user".to_string();
        let token_hash = create_sha256_hash("token");

        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: token_hash.clone(),
                    username: "user".to_string(),
                    expires_at: chrono::Utc::now().timestamp() + 3600,
                    created_at: chrono::Utc::now().timestamp(),
                    is_revoked: false,
                }))
            });

        token_repo
            .expect_revoke_refresh_token()
            .times(1)
            .returning(|_| Err(sqlx::Error::RowNotFound));

        let result = refresh("token", &token_repo, &user_repo, &private_key, &username).await;

        assert!(
            matches!(result, Err(AppError::InternalServerError(msg)) if msg == "An internal server error occurred")
        );
    }

    #[tokio::test]
    async fn test_refresh_user_not_found() {
        let mut user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();
        let username = "user".to_string();
        let token_hash = create_sha256_hash("token");

        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: token_hash.clone(),
                    username: "user".to_string(),
                    expires_at: chrono::Utc::now().timestamp() + 3600,
                    created_at: chrono::Utc::now().timestamp(),
                    is_revoked: false,
                }))
            });

        token_repo
            .expect_revoke_refresh_token()
            .returning(|_| Ok(1));

        user_repo
            .expect_get_user_by_name()
            .times(1)
            .returning(|_| Ok(None));

        let result = refresh("token", &token_repo, &user_repo, &private_key, &username).await;

        assert!(
            matches!(result, Err(AppError::InternalServerError(msg)) if msg == "An internal server error occurred")
        );
    }

    #[tokio::test]
    async fn test_refresh_invalid_rsa_key() {
        let mut user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "invalid-key".to_string();
        let username = "user".to_string();
        let token_hash = create_sha256_hash("token");

        token_repo.expect_get_refresh_token().returning(move |_| {
            Ok(Some(RefreshToken {
                token_hash: token_hash.clone(),
                username: "user".to_string(),
                expires_at: chrono::Utc::now().timestamp() + 3600,
                created_at: chrono::Utc::now().timestamp(),
                is_revoked: false,
            }))
        });

        token_repo
            .expect_revoke_refresh_token()
            .returning(|_| Ok(1));
        user_repo.expect_get_user_by_name().returning(|_| {
            Ok(Some(User {
                username: "user".to_string(),
                password_hash: "hash".to_string(),
                is_admin: false,
            }))
        });

        let result = refresh("token", &token_repo, &user_repo, &private_key, &username).await;

        assert!(
            matches!(result, Err(AppError::InternalServerError(msg)) if msg == "Token creation failed")
        );
    }

    #[tokio::test]
    async fn test_refresh_token_not_found_forged() {
        let user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let private_key = "dummy".to_string();
        let username = "user".to_string();

        // Simulate DB returning None (token doesn't exist/is forged)
        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(|_| Ok(None));

        let result = refresh(
            "forged_token",
            &token_repo,
            &user_repo,
            &private_key,
            &username,
        )
        .await;

        assert!(matches!(result, Err(AppError::InvalidRefreshToken)));
    }

    #[tokio::test]
    async fn test_refresh_token_expiration_boundary() {
        let mut user_repo = MockUserRepository::new();
        let mut token_repo = MockRefreshTokenRepository::new();
        let username = "user".to_string();

        let now = chrono::Utc::now().timestamp();

        // Case 1: Token expires EXACTLY now.
        // Logic: if expires_at < current_time { Error }
        // So expires_at == now should technically still be valid for that exact second.
        token_repo
            .expect_get_refresh_token()
            .times(1)
            .returning(move |_| {
                Ok(Some(RefreshToken {
                    token_hash: "hash".to_string(),
                    username: "user".to_string(),
                    expires_at: now, // Exactly at current time
                    created_at: now - 3600,
                    is_revoked: false,
                }))
            });

        // If it's accepted, it will try to revoke it
        token_repo
            .expect_revoke_refresh_token()
            .returning(|_| Ok(1));
        user_repo.expect_get_user_by_name().returning(move |_| {
            Ok(Some(User {
                username: "user".to_string(),
                password_hash: "hash".to_string(),
                is_admin: false,
            }))
        });
        token_repo
            .expect_create_refresh_token()
            .returning(|_| Ok(1));

        let result = refresh(
            "token",
            &token_repo,
            &user_repo,
            get_test_rsa_key(),
            &username,
        )
        .await;

        // This confirms the boundary behavior: exact match is the last valid second.
        assert!(
            result.is_ok(),
            "Token expiring exactly now should be valid for the duration of this second"
        );
    }
}
