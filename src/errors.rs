// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InternalServerError(String),
    NotFound,
    Conflict(String),
    // You can add more here later, e.g., ConfigError, DatabaseConnectionError...
}

// We still implement IntoResponse here so it can be used directly in handlers.
// (In very large apps, you might split this, but for now, this is perfect).
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}