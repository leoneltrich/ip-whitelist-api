use crate::api::services::auth;
use crate::models::api::auth::{LoginRequest, LoginResponse, LogoutRequest, LogoutResponse};
use crate::state::AppState;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Extension, Json};
use shared::auth::models::Claims;
use shared::errors::app_errors::AppError;

#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "An internal server error occurred")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = auth::login(
        &*state.repositories.user,
        &*state.repositories.refresh_token,
        &state.config.private_key_pem,
        payload,
    )
    .await?;

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logout successful", body = LogoutResponse),
        (status = 401, description = "You are not logged in"),
        (status = 500, description = "An internal server error occurred")
    )
)]
pub(crate) async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = auth::logout(
        &*state.repositories.refresh_token,
        &claims.sub,
        &payload.refresh_token,
    )
    .await?;

    Ok((StatusCode::OK, Json(response)))
}
