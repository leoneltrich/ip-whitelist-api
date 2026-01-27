use crate::api::services::auth;
use crate::state::AppState;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use shared::auth::models::{LoginRequest, LoginResponse};
use shared::errors::AppError;

#[utoipa::path(
    post,
    path = "/api/v1/login",
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
        Json(response),
    ))
}
