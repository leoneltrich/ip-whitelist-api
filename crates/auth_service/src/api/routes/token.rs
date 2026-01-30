use crate::api::services::auth;
use crate::models::api::auth::{LogoutRequest, TokenRefreshRequest, TokenRefreshResponse};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use shared::auth::models::Claims;
use shared::errors::AppError;

#[utoipa::path(
    post,
    path = "/api/v1/token/refresh",
    request_body = TokenRefreshRequest,
    responses(
        (status = 200, description = "Refresh successful", body = TokenRefreshResponse),
        (status = 401, description = "Invalid refresh token"),
        (status = 500, description = "An internal server error occurred")
    )
)]
pub(crate) async fn refresh(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = auth::token::refresh(
        &payload.refresh_token,
        &*state.repositories.refresh_token,
        &*state.repositories.user,
        &state.config.private_key_pem,
        &claims.sub,
    )
    .await?;
    Ok((StatusCode::OK, Json(response)))
}
