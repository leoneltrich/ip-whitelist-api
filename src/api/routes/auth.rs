use crate::api::services::auth;
use crate::errors::AppError;
use crate::models::api::auth::LoginRequest;
use crate::state::AppState;
use axum::http::StatusCode;
// src/api/routes/auth.rs
use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;

// This handler now lives in its own home
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = auth::login(&state, payload).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "you are logged in",
            "token": response.token
        })),
    ))
}
