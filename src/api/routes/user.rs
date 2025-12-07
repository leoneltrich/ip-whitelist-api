use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json, response::IntoResponse,
};

// 1. Import the Repository Interface
use crate::persistence::repository::Repositories;

// 2. Import the Domain Entity (for talking to the Repo)
use crate::model::database::user::User;

// 3. Import the DTOs (for talking to the HTTP Client)
//    We now point to model -> api -> user
use crate::model::api::user::{CreateUserRequest, UpdateUserRequest};

// --- POST: Create User ---
pub async fn create_user(
    State(repos): State<Repositories>,
    Json(payload): Json<CreateUserRequest>, // Uses the DTO
) -> impl IntoResponse {

    // Map DTO -> Domain Entity
    // In a real app, you would likely hash the password here
    let user = User {
        username: payload.username,
        password_hash: payload.password,
    };

    match repos.user.create_user(&user).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// --- PUT: Update User ---
pub async fn update_user(
    State(repos): State<Repositories>,
    Json(payload): Json<UpdateUserRequest>, // Uses the DTO
) -> impl IntoResponse {

    // Map DTO -> Domain Entity
    let user = User {
        username: payload.username,
        password_hash: payload.password,
    };

    match repos.user.update_user(&user).await {
        Ok(rows) => {
            if rows == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::OK
            }
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// --- DELETE: Delete User ---
// (No DTO needed here, we just use the URL path)
pub async fn delete_user(
    State(repos): State<Repositories>,
    Path(username): Path<String>,
) -> impl IntoResponse {

    match repos.user.delete_user(&username).await {
        Ok(rows) => {
            if rows == 0 {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::NO_CONTENT
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}