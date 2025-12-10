use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub servername: String,
    pub port: u16,
    pub api_startup_method: Option<String>,
    pub api_startup_link: Option<String>,
    pub api_startup_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
    pub servername: String,
    pub port: u16,
    pub api_startup_method: Option<String>,
    pub api_startup_link: Option<String>,
    pub api_startup_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServerResponse {
    pub servername: String,
    pub port: u16,
    pub api_startup_method: Option<String>,
    pub api_startup_link: Option<String>,
    // Professional security practice: Do not return secrets (tokens) in GET requests.
    // Instead, return a flag indicating if one is set.
    pub has_token: bool,
}