use crate::auth::models::Claims;
use crate::errors::AppError;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

/// Signs a JWT using the RS256 private key (PEM format).
pub fn sign(claims: Claims, private_key_pem: &str) -> Result<String, AppError> {
    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
        .map_err(|e| AppError::InternalServerError("Token creation failed".to_string()))?;

    encode(&Header::new(Algorithm::RS256), &claims, &key)
        .map_err(|e| AppError::InternalServerError("Token creation failed".to_string()))
}

/// Verifies a JWT using the RS256 public key (PEM format).
pub fn verify(token: &str, public_key_pem: &str) -> Result<Claims, AppError> {
    let key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
        .map_err(|e| AppError::InternalServerError("Token verification failed".to_string()))?;

    let validation = Validation::new(Algorithm::RS256);

    match decode::<Claims>(token, &key, &validation) {
        Ok(data) => Ok(data.claims),
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => Err(AppError::TokenExpired), // New variant
            _ => Err(AppError::InvalidToken),
        },
    }
}
