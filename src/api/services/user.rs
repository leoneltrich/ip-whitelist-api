use crate::errors::AppError;
// src/api/services/user.rs
use crate::models::api::user::{CreateUserRequest, UpdateUserRequest};
use crate::models::database::user::User;
use crate::persistence::repository::Repositories;

pub async fn create_user(repos: &Repositories, req: CreateUserRequest) -> Result<(), AppError> {
    let user = User {
        username: req.username,
        password_hash: req.password,
    };

    // We map the String error from the repo to our AppError
    repos.user.create_user(&user).await
        .map_err(|e| AppError::InternalServerError(e))?; // ? propagates the error

    Ok(())
}

pub async fn update_user(repos: &Repositories, req: UpdateUserRequest) -> Result<(), AppError> {
    let user = User {
        username: req.username,
        password_hash: req.password,
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