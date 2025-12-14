use serde::{Deserialize, Serialize};
use utoipa::ToSchema; // <--- Import ToSchema

#[derive(Debug, Deserialize, ToSchema)] // <--- Add ToSchema
pub struct AccessRequest {
    #[schema(example = "media-server-01")]
    pub server_id: String,
}

#[derive(Debug, Serialize, ToSchema)] // <--- Add ToSchema
pub struct AccessResponse {
    #[schema(example = "success")]
    pub status: String,
    #[schema(example = "IP 192.168.1.5 has been whitelisted.")]
    pub message: String,
    #[schema(example = "12h")]
    pub expires_in: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AccessStatusResponse {
    #[schema(example = "minecraft-survival")]
    pub server: String,

    #[schema(example = "192.168.1.50")]
    pub ip: String,

    #[schema(example = true)]
    pub is_active: bool,

    #[schema(example = 1735689600, nullable = true)]
    pub expiration: Option<i64>, // Unix timestamp, null if no access

    #[schema(example = "2h 30m", nullable = true)]
    pub time_remaining: Option<String>, // Human readable helper
}