use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)] // <--- Add ToSchema
pub struct CreateServerRequest {
    #[schema(example = "media-server-01")]
    pub servername: String,

    #[schema(example = 8080)]
    pub port: u16,

    #[schema(example = "TCP, UDP, TCP/UDP")]
    pub protocol: String,
}

#[derive(Debug, Deserialize, ToSchema)] // <--- Add ToSchema
pub struct UpdateServerRequest {
    #[schema(example = "media-server-01")]
    pub servername: String,

    #[schema(example = 8080)]
    pub port: u16,

    #[schema(example = "TCP, UDP, TCP/UDP")]
    pub protocol: String,
}

#[derive(Debug, Serialize, ToSchema)] // <--- Add ToSchema
pub struct ServerResponse {
    #[schema(example = "media-server-01")]
    pub servername: String,

    #[schema(example = 8080)]
    pub port: u16,

    #[schema(example = "TCP, UDP, TCP/UDP")]
    pub protocol: String,

}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerExistsResponse {
    #[schema(example = true)]
    pub exists: bool,
}
