// src/api/routes/auth.rs
use axum::{
    extract::State,
    Json,
    response::IntoResponse,
};
use crate::api::services::auth;
use crate::persistence::repository::Repositories;
use crate::models::api::auth::LoginRequest;
use crate::errors::AppError;

// This handler now lives in its own home
pub async fn login(
    State(repos): State<Repositories>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    
    let response = auth::login(&repos, payload).await?;

    Ok(Json(response))
}