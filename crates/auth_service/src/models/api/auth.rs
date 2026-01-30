use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "admin")]
    pub username: String,
    #[schema(example = "supersecret123")]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    #[schema(example = "74789D50D50B0568B4132AC53976574361ED218057A0E7A82523918B78589A61")]
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct LogoutResponse {
    #[schema(example = true)]
    pub success: bool,
    #[schema(example = "74789D50D50B0568B4132AC53976574361ED218057A0E7A82523918B78589A61")]
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LogoutRequest {
    #[schema(example = "74789D50D50B0568B4132AC53976574361ED218057A0E7A82523918B78589A61")]
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct TokenRefreshRequest {
    #[schema(example = "74789D50D50B0568B4132AC53976574361ED218057A0E7A82523918B78589A61")]
    pub refresh_token: String,
    pub username: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenRefreshResponse {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    #[schema(example = "74789D50D50B0568B4132AC53976574361ED218057A0E7A82523918B78589A61")]
    pub refresh_token: String,
}