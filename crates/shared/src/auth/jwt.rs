use crate::auth::models::Claims;
use crate::errors::app_errors::AppError;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use tracing::debug;

/// Signs a JWT using the RS256 private key (PEM format).
pub fn sign(claims: Claims, private_key_pem: &str) -> Result<String, AppError> {
    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes()).map_err(|e| {
        error!("Failed to encode RSA key: {}", e);
        AppError::InternalServerError
    })?;

    encode(&Header::new(Algorithm::RS256), &claims, &key).map_err(|e| {
        error!("Failed to create JWT");
        AppError::InternalServerError
    })
}

/// Verifies a JWT using the RS256 public key (PEM format).
pub fn verify(token: &str, public_key_pem: &str) -> Result<Claims, AppError> {
    debug!("Decoding JWT...");
    let key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).map_err(|e| {
        error!("Failed to decode JWT: {}", e);
        AppError::InternalServerError
    })?;

    debug!("Validating JWT...");
    let validation = Validation::new(Algorithm::RS256);

    match decode::<Claims>(token, &key, &validation) {
        Ok(data) => Ok(data.claims),
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => Err(AppError::TokenExpired),
            _ => Err(AppError::InvalidAccessToken),
        },
    }
}
