use crate::auth::models::Claims;
use crate::errors::AppError;
use crate::auth::jwt;

/// Verifies an "Authorization" header value and returns the claims.
/// This is pure business logic, decoupled from any web framework.
pub fn verify_token_from_header(auth_header: &str, public_key: &str) -> Result<Claims, AppError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::BadRequest);
    }

    let token = &auth_header[7..];

    jwt::verify(token, public_key)
}
