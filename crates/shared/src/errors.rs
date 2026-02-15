// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InternalServerError,
    ResourceNotFound,
    Conflict,
    InvalidCredentials,
    InvalidAccessToken,
    PermissionDenied,
    BadRequest,
    TokenExpired,
    InvalidRefreshToken,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    /// The machine-readable error code slug
    pub error: ErrorCode,
    /// A human-readable message providing more context
    #[schema(example = "The provided refresh token is expired or revoked")]
    pub message: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    InternalServerError(String),
    NotFound,
    Conflict(String),
    InvalidCredentials,
    InvalidToken,
    Forbidden,
    BadRequest(String),
    TokenExpired,
    InvalidRefreshToken,
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
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
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
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::InvalidCredentials,
                None,
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::InvalidAccessToken,
                None,
            ),

            AppError::Forbidden => (StatusCode::FORBIDDEN, ErrorCode::PermissionDenied, None),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, ErrorCode::BadRequest, Some(msg)),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, ErrorCode::TokenExpired, None),
            AppError::InvalidRefreshToken => (
                StatusCode::UNAUTHORIZED,
                ErrorCode::InvalidRefreshToken,
                None,
            ),
        };

        let body = Json(ErrorResponse {
            error: code,
            message: msg,
        });

        (status, body).into_response()
    }
}
