use crate::models::api::token::TokenExpiresResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use shared::auth::models::Claims;
use shared::errors::AppError;

#[utoipa::path(
    get,
    path = "/api/v1/token/expires",
    responses(
        (status = 200, description = "Timestamp of expiry", body = TokenExpiresResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
pub async fn expires(Extension(claims): Extension<Claims>) -> Result<impl IntoResponse, AppError> {
    let response = TokenExpiresResponse {
        expires_at: claims.exp,
    };

    Ok(Json(response))
}
