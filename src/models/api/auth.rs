use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

// Token is valid for 24 hours
const EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // "Subject" (Username)
    pub iat: usize,  // "Issued At" (Timestamp)
    pub exp: usize,  // "Expiration" (Timestamp)
}

impl Claims {
    pub fn new(username: String) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(EXPIRATION_HOURS);

        Self {
            sub: username,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}