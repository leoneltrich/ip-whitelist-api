use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};

// Token is valid for 24 hours
const EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub is_admin: bool,
}

impl Claims {
    pub fn new(username: String, is_admin: bool) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::hours(EXPIRATION_HOURS);

        Self {
            sub: username,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
            is_admin,
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