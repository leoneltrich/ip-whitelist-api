use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
const EXPIRATION_MINUTES: i64 = 10;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// User ID
    pub sub: String,
    /// Issued at
    pub iat: usize,
    /// Expiration time
    pub exp: usize,
    /// Does user have admin privileges?
    pub is_admin: bool,
}

impl Claims {
    pub fn new(username: String, is_admin: bool) -> Self {
        let now = Utc::now();
        let expiration = now + Duration::minutes(EXPIRATION_MINUTES);

        Self {
            sub: username,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
            is_admin,
        }
    }
}


