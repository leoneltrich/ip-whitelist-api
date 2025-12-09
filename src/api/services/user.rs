use bcrypt::{hash, DEFAULT_COST};
use crate::errors::AppError;
// src/api/services/user.rs
use crate::models::api::user::{CreateUserRequest, UpdateUserRequest};
use crate::models::database::user::User;
use crate::persistence::repository::Repositories;

pub async fn create_user(repos: &Repositories, req: CreateUserRequest) -> Result<(), AppError> {

    if repos.user.get_user(&req.username).await.is_ok() {
        return Err(AppError::Conflict(format!("User {} already exists", req.username)));
    }

    let password_hash = hash(req.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))?;


    let user = User {
        username: req.username,
        password_hash,
    };

    repos.user.create_user(&user).await
        .map_err(|e| AppError::InternalServerError(e))?;

    Ok(())
}

pub async fn update_user(repos: &Repositories, req: UpdateUserRequest) -> Result<(), AppError> {

    let password_hash = hash(req.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))?;

    let user = User {
        username: req.username,
        password_hash,
    };

    let rows = repos.user.update_user(&user).await
        .map_err(|e| AppError::InternalServerError(e))?;

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

pub async fn delete_user(repos: &Repositories, username: String) -> Result<(), AppError> {
    let rows = repos.user.delete_user(&username).await
        .map_err(|e| AppError::InternalServerError(e))?;

    if rows == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}