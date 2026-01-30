use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
// Token is valid for 24 hours
const EXPIRATION_HOURS: i64 = 24;

// Claims is internal logic (payload of the JWT), so we don't necessarily
// need to expose it in the OpenAPI schema unless you have an endpoint
// that returns raw claims.
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
        let expiration = now + Duration::hours(EXPIRATION_HOURS);

        Self {
            sub: username,
            iat: now.timestamp() as usize,
            exp: expiration.timestamp() as usize,
            is_admin,
        }
    }
}


