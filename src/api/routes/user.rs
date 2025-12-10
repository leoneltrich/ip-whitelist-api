// Import the service module
use crate::api::services::user as user_service;
use crate::errors::AppError;
use crate::models::api::auth::Claims;
use crate::models::api::user::{CreateUserRequest, UpdateProfileRequest, UpdateUserRequest};
use crate::state::AppState;
// src/api/routes/user.rs
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

// --- POST: Create User ---
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // <--- Changed return type

    // If this fails, the '?' automatically converts the error to an HTTP Response
    user_service::create_user(&state, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "created",
            "message": "User successfully created"
        })),
    ))
}

// PUT
pub async fn admin_update_user(
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    user_service::update_user(&state, payload).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "User data updated successfully"
        })),
    ))
}

pub async fn self_update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let trusted_request = UpdateUserRequest {
        username: claims.sub,
        is_admin: claims.is_admin,
        password: payload.password,
    };

    user_service::update_user(&state, trusted_request).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Profile updated successfully"
        })),
    ))
}

// DELETE
pub async fn delete_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    user_service::delete_user(&state, username).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!({
            "status": "deleted",
            "message": "User successfully deleted"
        })),
    ))
}
