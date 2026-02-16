// src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;
use utoipa::ToSchema;

/// The "Master" list of all possible error codes in the system.
/// This is used for the actual JSON serialization at runtime.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InternalServerError,
    ResourceNotFound,
    Conflict,
    BadRequest,
    InvalidCredentials,
    InvalidAccessToken,
    PermissionDenied,
    TokenExpired,
    InvalidRefreshToken,
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CommonErrorCodes {
    InternalServerError,
    ResourceNotFound,
    Conflict,
    BadRequest,
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LoginSpecificCodes {
    InvalidCredentials,
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RefreshSpecificErrorCodes {
    InvalidRefreshToken,
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SecuredErrorCodes {
    InvalidAccessToken,
    PermissionDenied,
    TokenExpired,
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum PublicErrorCodes {
    Common(CommonErrorCodes),
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum ProtectedErrorCodes {
    Common(CommonErrorCodes),
    Secured(SecuredErrorCodes),
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum RefreshErrorCodes {
    Common(CommonErrorCodes),
    Refresh(RefreshSpecificErrorCodes),
}

#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum LoginErrorCodes {
    Common(CommonErrorCodes),
    Login(LoginSpecificCodes),
}

/// The generic wrapper for all error responses in the OpenAPI spec.
/// Use ErrorResponse<LoginErrorCodes>, etc., in your route documentation.
#[derive(Serialize, ToSchema, Debug)]
pub struct ErrorResponse<T: ToSchema> {
    /// The machine-readable error code slug
    pub error: T,
    /// A human-readable message providing more context
    #[schema(example = "The provided refresh token is expired or revoked")]
    pub message: Option<String>,
}

/// The actual struct returned by into_response at runtime.
/// This avoids needing to specify a generic type in the IntoResponse implementation.
#[derive(Serialize, Debug)]
pub struct MainErrorResponse {
    /// The machine-readable error code slug
    pub error: ErrorCode,
    /// A human-readable message providing more context
    pub message: Option<String>,
}

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
