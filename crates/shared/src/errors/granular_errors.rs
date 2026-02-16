use serde::Serialize;
use utoipa::ToSchema;

/// Error code for 500 Internal Server Error.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum InternalServerErrorCodes {
    /// An unexpected error occurred on the server.
    InternalServerError,
}

/// Error code for 404 Not Found.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum NotFoundErrorCodes {
    /// The requested resource could not be found.
    ResourceNotFound,
}

/// Error code for 409 Conflict.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ConflictErrorCodes {
    /// The request could not be completed due to a conflict with the current state of the resource.
    Conflict,
}

/// Error code for 400 Bad Request.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BadRequestErrorCodes {
    /// The server could not understand the request due to invalid syntax.
    BadRequest,
}

/// Error code for 403 Forbidden.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PermissionErrorCodes {
    /// The server understood the request but refuses to authorize it.
    PermissionDenied,
}

/// Error code for 401 Unauthorized specifically for the /login endpoint.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LoginAuthErrorCodes {
    /// The provided credentials (e.g., username/password) are incorrect.
    InvalidCredentials,
}

/// Error code for 401 Unauthorized specifically for token refresh.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TokenRefreshErrorCodes {
    /// The provided refresh token is invalid, expired, or revoked.
    InvalidRefreshToken,
}

/// Error codes for 401 Unauthorized on generic protected routes.
#[derive(Serialize, ToSchema, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum AccessAuthErrorCodes {
    /// The access token is missing or malformed.
    InvalidAccessToken,
    /// The access token has expired.
    TokenExpired,
}