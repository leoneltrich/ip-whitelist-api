use crate::errors::AppError;
use bcrypt::{DEFAULT_COST, hash, verify};

/// Hash a password using the standard application configuration.
pub fn hash_password(password: String) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))
}

/// Verify a password against a stored hash.
/// Returns Ok(true) if valid, Ok(false) if invalid.
/// Returns Err only on internal crypto failure (not on wrong password).
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|e| AppError::InternalServerError(format!("Password verification failed: {}", e)))
}
