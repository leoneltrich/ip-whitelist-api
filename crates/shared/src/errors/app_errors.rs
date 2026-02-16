use crate::errors::utoipa_errors::ErrorCode;
use std::fmt;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// The main error type used throughout the application.
///
/// This enum implements `IntoResponse` to automatically convert errors
/// into appropriate HTTP responses with JSON bodies.
#[derive(Debug, Clone)]
pub enum AppError {
    InternalServerError(String),
    NotFound,
    Conflict(String),
    BadRequest(String),
    InvalidCredentials,
    InvalidAccessToken,
    PermissionDenied,
    TokenExpired,
    InvalidRefreshToken,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::NotFound => write!(f, "Resource Not Found"),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::InvalidCredentials => write!(f, "Invalid Credentials"),
            AppError::InvalidAccessToken => write!(f, "Invalid Access Token"),
            AppError::PermissionDenied => write!(f, "Permission Denied"),
            AppError::TokenExpired => write!(f, "Token Expired"),
            AppError::InvalidRefreshToken => write!(f, "Invalid Refresh Token"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, msg) = match self {
            AppError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ErrorCode::InternalServerError, Some(msg))
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, ErrorCode::ResourceNotFound, None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, ErrorCode::Conflict, Some(msg)),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, ErrorCode::BadRequest, Some(msg)),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::InvalidCredentials,
                None,
            ),
            AppError::InvalidAccessToken => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::InvalidAccessToken,
                None,
            ),
            AppError::PermissionDenied => (StatusCode::FORBIDDEN, ErrorCode::PermissionDenied, None),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, ErrorCode::TokenExpired, None),
            AppError::InvalidRefreshToken => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::InvalidRefreshToken,
                None,
            ),
        };

        let body = Json(MainErrorResponse {
            error: code,
            message: msg,
        });

        (status, body).into_response()
    }
}

/// The actual JSON body returned by the API at runtime.
#[derive(Serialize, Debug)]
pub struct MainErrorResponse {
    /// The machine-readable error code slug.
    pub error: ErrorCode,
    /// A human-readable message providing more context.
    pub message: Option<String>,
}