use crate::auth::jwt;
use crate::auth::models::Claims;
use crate::errors::app_errors::AppError;
use log::{debug, info};

/// Verifies an "Authorization" header value and returns the claims.
pub fn verify_token_from_header(auth_header: &str, public_key: &str) -> Result<Claims, AppError> {
    if !auth_header.starts_with("Bearer ") {
        info!("Received malformed auth header, must start with \"Bearer \"");
        return Err(AppError::BadRequest("Authorization header must start with \"Bearer \"".to_string()));
    }

    let token = &auth_header[7..];

    debug!("Verifying JWT token");
    jwt::verify(token, public_key)
}
