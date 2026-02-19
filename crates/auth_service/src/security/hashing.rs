use bcrypt::{hash, verify, DEFAULT_COST};
use sha2::{Digest, Sha256};
use shared::errors::app_errors::AppError;
use tracing::error;

/// Hash a password using the standard application configuration.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| {
            error!("An error occurred hashing the password: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })
}

/// Verify a password against a stored hash.
/// Returns Ok(true) if valid, Ok(false) if invalid.
/// Returns Err only on internal crypto failure (not on wrong password).
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|e| {
            error!("An error occurred verifying the password: {}", e);
            AppError::InternalServerError("An internal server error occurred".to_string())
        })
}

/// Create a SHA256 hash from a string.
pub(crate) fn create_sha256_hash(string: &str) -> String {
    let mut hasher = Sha256::new();
    Digest::update(&mut hasher, string.as_bytes());
    let hash = hasher.finalize();
    hex::encode(hash)
}
