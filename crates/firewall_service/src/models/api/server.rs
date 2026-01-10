use serde::{Deserialize, Serialize};
use utoipa::ToSchema; // <--- Import ToSchema

#[derive(Debug, Deserialize, ToSchema)] // <--- Add ToSchema
pub struct CreateServerRequest {
    #[schema(example = "media-server-01")]
    pub servername: String,

    #[schema(example = 8080)]
    pub port: u16,

    #[schema(example = "POST")]
    pub api_startup_method: Option<String>,

    #[schema(example = "http://192.168.1.50:5000/start")]
    pub api_startup_link: Option<String>,

    #[schema(example = "secret-startup-token")]
    pub api_startup_token: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)] // <--- Add ToSchema
pub struct UpdateServerRequest {
    #[schema(example = "media-server-01")]
    pub servername: String,

    #[schema(example = 8080)]
    pub port: u16,

    #[schema(example = "POST")]
    pub api_startup_method: Option<String>,

    #[schema(example = "http://192.168.1.50:5000/start")]
    pub api_startup_link: Option<String>,

    #[schema(example = "secret-startup-token")]
    pub api_startup_token: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)] // <--- Add ToSchema
pub struct ServerResponse {
    #[schema(example = "media-server-01")]
    pub servername: String,

    #[schema(example = 8080)]
    pub port: u16,

    #[schema(example = "POST")]
    pub api_startup_method: Option<String>,

    #[schema(example = "http://192.168.1.50:5000/start")]
    pub api_startup_link: Option<String>,

    #[schema(example = true)]
    pub has_token: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerExistsResponse {
    #[schema(example = true)]
    pub exists: bool,
}