// src/api/routes/user.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, response::IntoResponse,
};
use crate::persistence::repository::Repositories;
use crate::models::api::user::{CreateUserRequest, UpdateUserRequest};
// Import the service module
use crate::api::services::user as user_service;
use crate::errors::AppError;
use crate::state::AppState;

// --- POST: Create User ---
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> { // <--- Changed return type

    // If this fails, the '?' automatically converts the error to an HTTP Response
    user_service::create_user(&state, payload).await?;

    Ok(StatusCode::CREATED)
}

// PUT
pub async fn update_user(
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {

    user_service::update_user(&state, payload).await?;

    Ok(StatusCode::OK)
}

// DELETE
pub async fn delete_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {

    user_service::delete_user(&state, username).await?;

    Ok(StatusCode::NO_CONTENT)
}