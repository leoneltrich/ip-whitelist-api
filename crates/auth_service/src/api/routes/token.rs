use crate::api::services::auth;
use crate::models::api::auth::{TokenRefreshRequest, TokenRefreshResponse};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use shared::errors::app_errors::AppError;
use shared::errors::utoipa_errors::{BadRequestErrorResponse, InternalServerErrorResponse, TokenRefreshErrorResponse};

#[utoipa::path(
    post,
    path = "/api/v1/token/refresh",
    request_body = TokenRefreshRequest,
    responses(
        (status = 200, description = "Refresh successful", body = TokenRefreshResponse),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Invalid refresh token", body = TokenRefreshErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    tags = ["Auth"]
)]
pub(crate) async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<TokenRefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = auth::token::refresh(
        &payload.refresh_token,
        &*state.repositories.refresh_token,
        &*state.repositories.user,
        &state.config.private_key_pem,
        &payload.username,
    )
    .await?;
    Ok((StatusCode::OK, Json(response)))
}
