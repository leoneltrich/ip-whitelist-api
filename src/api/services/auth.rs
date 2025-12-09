// src/api/services/auth.rs

// 1. Imports from your pluralized modules
use crate::models::api::auth::{LoginRequest, LoginResponse};
use crate::persistence::repository::Repositories; // assuming 'repository' module is still singular based on your earlier snippet
use crate::errors::AppError;

// 2. Security imports
use bcrypt::verify;
use uuid::Uuid;

pub async fn login(repos: &Repositories, req: LoginRequest) -> Result<LoginResponse, AppError> {
    // 1. Attempt to fetch the user
    let user_option = repos.user.get_user(&req.username).await
        .map_err(|e| AppError::InternalServerError(e))?;

    // 2. Check if user exists (Security: Handle generic error)
    let user = match user_option {
        Some(u) => u,
        None => return Err(AppError::InvalidCredentials),
    };

    // 3. Verify the password hash
    let is_valid = verify(req.password, &user.password_hash)
        .map_err(|e| AppError::InternalServerError(format!("Verification failed: {}", e)))?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // 4. Generate a simple random token (UUID)
    // NOTE: In a real app, you would save this token to a DB table (sessions) 
    // to track it. For now, we just return it.
    let token = Uuid::new_v4().to_string();

    Ok(LoginResponse { token })
}