use serde::Deserialize;
use utoipa::ToSchema; // <--- Import ToSchema

#[derive(Deserialize, ToSchema)] // <--- Add ToSchema
pub struct CreateUserRequest {
    #[schema(example = "new_user")]
    pub username: String,

    #[schema(example = "password123")]
    pub password: String,

    #[schema(example = false)]
    pub is_admin: bool,
}

#[derive(Deserialize, ToSchema)] // <--- Add ToSchema
pub struct UpdateUserRequest {
    #[schema(example = "target_user")]
    pub username: String,

    #[schema(example = "new_password123")]
    pub password: String,

    #[schema(example = false)]
    pub is_admin: bool,
}

#[derive(Deserialize, ToSchema)] // <--- Add ToSchema
pub struct UpdateProfileRequest {
    #[schema(example = "my_new_secure_password")]
    pub password: String,
}