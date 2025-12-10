use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String, // The API receives a raw password
    pub is_admin: bool,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: String, // We need to know who to update
    pub password: String,
    pub is_admin: bool,
}