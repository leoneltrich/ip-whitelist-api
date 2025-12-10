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