use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::models::database::user::User;
// <--- Import ToSchema

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    #[schema(example = "admin_user")]
    pub username: String,

    #[schema(example = true)]
    pub is_admin: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            is_admin: user.is_admin,
        }
    }
}

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