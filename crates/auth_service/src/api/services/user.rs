use crate::models::api::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::models::database::user::User;
use crate::security::hashing;
use crate::state::AppState;
use shared::errors::AppError;

pub async fn create_user(state: &AppState, req: CreateUserRequest) -> Result<(), AppError> {
    if state
        .repositories
        .user
        .get_user_by_name(&req.username)
        .await
        .unwrap()
        .is_none()
        == false
    {
        return Err(AppError::Conflict(format!(
            "User {} already exists",
            req.username
        )));
    }

    let password_hash = hashing::hash_password(&req.password)
        .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

    let user = User {
        username: req.username,
        password_hash,
        is_admin: req.is_admin,
    };

    state
        .repositories
        .user
        .create_user(&user)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(())
}

pub async fn update_user(state: &AppState, req: UpdateUserRequest) -> Result<(), AppError> {
    let password_hash = hashing::hash_password(&req.password)
        .map_err(|_| AppError::InternalServerError("Password hashing failed".to_string()))?;

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
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn delete_user(state: &AppState, username: String) -> Result<(), AppError> {
    let rows = state
        .repositories
        .user
        .delete_user(&username)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn get_all_users(state: &AppState) -> Result<Vec<UserResponse>, AppError> {
    let users = state
        .repositories
        .user
        .get_all_users()
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(response)
}
