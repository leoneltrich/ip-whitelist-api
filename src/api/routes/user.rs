// Import the service module
use crate::api::services::user as user_service;
use crate::errors::AppError;
use crate::models::api::auth::Claims;
use crate::models::api::user::{CreateUserRequest, UpdateProfileRequest, UpdateUserRequest, UserListResponse};
use crate::state::AppState;
// src/api/routes/user.rs
use axum::{
    extract::{Path, State}, http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use serde_json::json;

// Self-routes

#[utoipa::path(
    put,
    path = "/users/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated"),
        (status = 401, description = "Unauthorized")
    ),
    security(("jwt" = []))
)]
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

// Admin routes

#[utoipa::path(
    post,
    path = "/admin/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created"),
        (status = 409, description = "Username already exists"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {

    user_service::create_user(&state, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "created",
            "message": "User successfully created"
        })),
    ))
}

#[utoipa::path(
    put,
    path = "/admin/users",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated"),
        (status = 404, description = "User not found"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
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

#[utoipa::path(
    delete,
    path = "/admin/users/{username}",
    params(
        ("username" = String, Path, description = "Username to delete")
    ),
    responses(
        (status = 204, description = "User deleted"), // 204 No Content or 200 OK depending on impl
        (status = 404, description = "User not found"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
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


#[utoipa::path(
    get,
    path = "/admin/users",
    responses(
        (status = 200, description = "List of all users retrieved successfully", body = UserListResponse),
        (status = 500, description = "Internal Server Error"),
        (status = 401, description = "Unauthorized")
    ),
    security(("jwt" = []))
)]
pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {

    let users = user_service::get_all_users(&state).await?;

    let response = UserListResponse {
        status: "success".to_string(),
        data: users,
    };

    Ok((
        StatusCode::OK,
        Json(response),
    ))
}