// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InternalServerError(String),
    NotFound,
    Conflict(String),
    InvalidCredentials,
    InvalidToken,
    Forbidden,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::NotFound => write!(f, "Resource Not Found"),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::InvalidCredentials => write!(f, "Invalid Credentials"),
            AppError::InvalidToken => write!(f, "Invalid Token"),
            AppError::Forbidden => write!(f, "Forbidden"),
        }
    }
}

impl std::error::Error for AppError {}

// We still implement IntoResponse here so it can be used directly in handlers.
// (In very large apps, you might split this, but for now, this is perfect).
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid, expired or missing token".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Permission denied".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}