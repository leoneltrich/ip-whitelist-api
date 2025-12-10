use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AccessRequest {
    pub server_id: String,
}

#[derive(Debug, Serialize)]
pub struct AccessResponse {
    pub status: String,
    pub message: String,
    pub expires_in: String,
}
