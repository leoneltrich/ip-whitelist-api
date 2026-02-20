use crate::models::api::user::{
    CreateUserRequest, UpdateProfileRequest, UpdateUserRequest, UserResponse,
};
use crate::models::database::user::User;
use crate::security::hashing;
use crate::state::AppState;
use shared::auth::models::Claims;
use shared::errors::app_errors::AppError;
use tracing::{debug, error, info};

pub async fn create_user(state: &AppState, req: CreateUserRequest) -> Result<(), AppError> {
    let existing_user = state
        .repositories
        .user
        .get_user_by_name(&req.username)
        .await
        .map_err(|e| {
            error!("An error occurred accessing the database: {}", e);
            AppError::InternalServerError
        })?;

    if existing_user.is_some() {
        info!(
            "User {} already exists and won't be recreated",
            &req.username
        );
        return Err(AppError::Conflict(format!(
            "User {} already exists",
            req.username
        )));
    }

    let password_hash = hashing::hash_password(&req.password).map_err(|e| {
        error!("An error occurred hashing the password: {}", e);
        AppError::InternalServerError
    })?;

    let user = User {
        username: req.username,
        password_hash,
        is_admin: req.is_admin,
    };

    info!("Creating user {}", &user.username);
    state
        .repositories
        .user
        .create_user(&user)
        .await
        .map_err(|e| {
            error!(
                "An error occurred accessing the database creating the user: {}",
                e
            );
            AppError::InternalServerError
        })?;

    info!(
        "Successfully created user with username: {}",
        &user.username
    );
    Ok(())
}

pub async fn update_user(state: &AppState, req: UpdateUserRequest) -> Result<(), AppError> {
    let password_hash = hashing::hash_password(&req.password).map_err(|e| {
        error!("An error occurred hashing the password: {}", e);
        AppError::InternalServerError
    })?;

    let user = User {
        username: req.username,
        password_hash,
        is_admin: req.is_admin,
    };

    let rows = state
        .repositories
        .user
        .update_user(&user)
        .await
        .map_err(|e| {
            error!("An error occurred accessing the database: {}", e);
            AppError::InternalServerError
        })?;

    if rows == 0 {
        info!(
            "No user with username {} found in database.",
            &user.username
        );
        return Err(AppError::NotFound);
    }

    info!(
        "Successfully updated user with username: {}",
        &user.username
    );
    Ok(())
}

pub async fn delete_user(state: &AppState, username: String) -> Result<(), AppError> {
    let rows = state
        .repositories
        .user
        .delete_user(&username)
        .await
        .map_err(|e| {
            error!("An error occurred accessing the database: {}", e);
            AppError::InternalServerError
        })?;

    if rows == 0 {
        info!("No user with username {} found in database.", &username);
        return Err(AppError::NotFound);
    }

    info!("Successfully deleted user with username: {}", &username);
    Ok(())
}

pub async fn get_all_users(state: &AppState) -> Result<Vec<UserResponse>, AppError> {
    let users = state.repositories.user.get_all_users().await.map_err(|e| {
        error!("An error occurred accessing the database: {}", e);
        AppError::InternalServerError
    })?;

    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    debug!("Found {} users in database", response.len());

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::persistence::repository::interface::refresh_token::MockRefreshTokenRepository;
    use crate::persistence::repository::interface::user::MockUserRepository;
    use crate::persistence::repository::Repositories;
    use shared::logging::models::LogConfig;
    use std::sync::Arc;

    fn setup_test_state(user_repo: MockUserRepository) -> AppState {
        let repositories = Repositories {
            user: Arc::new(user_repo),
            refresh_token: Arc::new(MockRefreshTokenRepository::new()),
        };
        let config = AppConfig {
            private_key_pem: "dummy".to_string(),
            public_key_pem: "dummy".to_string(),
            database_path: "dummy".to_string(),
            log_config: LogConfig::new_dummy(),
        };
        AppState::new(config, repositories)
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let mut user_repo = MockUserRepository::new();
        let username = "newuser".to_string();

        user_repo
            .expect_get_user_by_name()
            .with(mockall::predicate::eq(username.clone()))
            .times(1)
            .returning(|_| Ok(None));

        user_repo.expect_create_user().times(1).returning(|_| Ok(1));

        let state = setup_test_state(user_repo);
        let req = CreateUserRequest {
            username: username.clone(),
            password: "password123".to_string(),
            is_admin: false,
        };

        let result = create_user(&state, req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_user_already_exists() {
        let mut user_repo = MockUserRepository::new();
        let username = "existinguser".to_string();

        user_repo
            .expect_get_user_by_name()
            .with(mockall::predicate::eq(username.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(User {
                    username: username.clone(),
                    password_hash: "hash".to_string(),
                    is_admin: false,
                }))
            });

        let state = setup_test_state(user_repo);
        let req = CreateUserRequest {
            username: "existinguser".to_string(),
            password: "password123".to_string(),
            is_admin: false,
        };

        let result = create_user(&state, req).await;
        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_update_user().times(1).returning(|_| Ok(1));

        let state = setup_test_state(user_repo);
        let req = UpdateUserRequest {
            username: "user".to_string(),
            password: "newpassword".to_string(),
            is_admin: true,
        };

        let result = update_user(&state, req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_update_user().times(1).returning(|_| Ok(0));

        let state = setup_test_state(user_repo);
        let req = UpdateUserRequest {
            username: "nonexistent".to_string(),
            password: "password".to_string(),
            is_admin: false,
        };

        let result = update_user(&state, req).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        let mut user_repo = MockUserRepository::new();
        user_repo
            .expect_delete_user()
            .with(mockall::predicate::eq("user".to_string()))
            .times(1)
            .returning(|_| Ok(1));

        let state = setup_test_state(user_repo);
        let result = delete_user(&state, "user".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_get_all_users().times(1).returning(|| {
            Ok(vec![
                User {
                    username: "u1".to_string(),
                    password_hash: "h1".to_string(),
                    is_admin: true,
                },
                User {
                    username: "u2".to_string(),
                    password_hash: "h2".to_string(),
                    is_admin: false,
                },
            ])
        });

        let state = setup_test_state(user_repo);
        let result = get_all_users(&state).await;

        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].username, "u1");
        assert!(users[0].is_admin);
    }

    #[tokio::test]
    async fn test_create_user_db_error_handling_no_panic() {
        let mut user_repo = MockUserRepository::new();

        // Simulate a database error (e.g., connection lost)
        user_repo
            .expect_get_user_by_name()
            .times(1)
            .returning(|_| Err(sqlx::Error::PoolClosed));

        let state = setup_test_state(user_repo);
        let req = CreateUserRequest {
            username: "any_user".into(),
            password: "any_password".into(),
            is_admin: false,
        };

        let result = create_user(&state, req).await;

        assert!(matches!(result, Err(AppError::InternalServerError)));
    }
}

pub async fn sanitize_user_self_update_request(
    state: &AppState,
    claims: &Claims,
    payload: UpdateProfileRequest,
) -> Result<UpdateUserRequest, AppError> {
    let current_user = state
        .repositories
        .user
        .get_user_by_name(&claims.sub)
        .await
        .map_err(|_| {
            AppError::InternalServerError
        })?
        .ok_or(AppError::NotFound)?;

    let trusted_request = UpdateUserRequest {
        username: claims.sub.clone(),
        is_admin: current_user.is_admin,
        password: payload.password,
    };
    Ok(trusted_request)
}
