use crate::errors::granular_errors::{
    AccessAuthErrorCodes, BadRequestErrorCodes, ConflictErrorCodes, InternalServerErrorCodes,
    LoginAuthErrorCodes, NotFoundErrorCodes, PermissionErrorCodes, TokenRefreshErrorCodes,
};
use serde::Serialize;
use utoipa::ToSchema;

/// A generic error response structure used across all services.
///
/// This structure ensures that all error responses follow the same format,
/// making it easier for clients to handle errors consistently.
#[derive(Serialize, ToSchema, Debug)]
pub struct ErrorResponse<T: ToSchema> {
    /// A machine-readable error code slug.
    /// Clients should use this field to programmatically handle the error.
    pub error: T,
    /// A human-readable message providing more context.
    /// This is intended for developers and can be used for logging or debugging.
    #[schema(example = "Detailed explanation of the error")]
    pub message: Option<String>,
}

/// Documentation schema for a 500 Internal Server Error response.
#[derive(ToSchema)]
#[schema(as = InternalServerError)]
pub struct InternalServerErrorResponse(ErrorResponse<InternalServerErrorCodes>);

/// Documentation schema for a 404 Not Found response.
#[derive(ToSchema)]
#[schema(as = NotFoundError)]
pub struct NotFoundErrorResponse(ErrorResponse<NotFoundErrorCodes>);

/// Documentation schema for a 409 Conflict response.
#[derive(ToSchema)]
#[schema(as = ConflictError)]
pub struct ConflictErrorResponse(ErrorResponse<ConflictErrorCodes>);

/// Documentation schema for a 400 Bad Request response.
#[derive(ToSchema)]
#[schema(as = BadRequestError)]
pub struct BadRequestErrorResponse(ErrorResponse<BadRequestErrorCodes>);

/// Documentation schema for a 403 Forbidden response.
#[derive(ToSchema)]
#[schema(as = PermissionError)]
pub struct PermissionErrorResponse(ErrorResponse<PermissionErrorCodes>);

/// Documentation schema for a 401 Unauthorized response during login.
#[derive(ToSchema)]
#[schema(as = LoginAuthError)]
pub struct LoginAuthErrorResponse(ErrorResponse<LoginAuthErrorCodes>);

/// Documentation schema for a 401 Unauthorized response during token refresh.
#[derive(ToSchema)]
#[schema(as = TokenRefreshError)]
pub struct TokenRefreshErrorResponse(ErrorResponse<TokenRefreshErrorCodes>);

/// Documentation schema for a 401 Unauthorized response on protected routes.
#[derive(ToSchema)]
#[schema(as = AccessAuthError)]
pub struct AccessAuthErrorResponse(ErrorResponse<AccessAuthErrorCodes>);

/// The complete list of all possible error codes in the system.
///
/// This enum is used at runtime to serialize the JSON response.
/// It should be kept in sync with the granular enums above.
#[derive(Serialize, ToSchema, Debug, Clone, Copy, PartialEq)]
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

