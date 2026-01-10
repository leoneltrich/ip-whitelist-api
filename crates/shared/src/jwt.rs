// crates/shared/src/jwt.rs
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use crate::auth_models::Claims;
use crate::errors::AppError;

/// Signs a JWT using the RS256 private key (PEM format).
pub fn sign(claims: Claims, private_key_pem: &str) -> Result<String, AppError> {
    let key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
        .map_err(|e| AppError::InternalServerError(format!("Invalid private key: {}", e)))?;
    
    encode(&Header::new(Algorithm::RS256), &claims, &key)
        .map_err(|e| AppError::InternalServerError(format!("Token creation failed: {}", e)))
}

/// Verifies a JWT using the RS256 public key (PEM format).
pub fn verify(token: &str, public_key_pem: &str) -> Result<Claims, AppError> {
    let key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
        .map_err(|e| AppError::InternalServerError(format!("Invalid public key: {}", e)))?;
        
    let validation = Validation::new(Algorithm::RS256);
    
    let token_data = decode::<Claims>(token, &key, &validation)
        .map_err(|_| AppError::InvalidToken)?;
        
    Ok(token_data.claims)
}
