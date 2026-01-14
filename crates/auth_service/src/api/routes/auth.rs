use crate::api::services::auth;
use crate::state::AppState;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use shared::auth::models::{LoginRequest, LoginResponse};
use shared::errors::AppError;

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
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
