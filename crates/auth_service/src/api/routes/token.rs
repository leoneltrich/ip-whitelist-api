use crate::models::api::auth::{LogoutRequest, TokenRefreshRequest, TokenRefreshResponse};
use crate::state::AppState;
use axum::extract::State;
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
pub(crate) fn refresh(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    todo!()
}
