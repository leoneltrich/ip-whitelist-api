use utoipa::OpenApi;
use crate::api::routes;
use shared::auth_models as auth;
use shared::health_models as health;
use crate::models::api::{token, user};

#[derive(OpenApi)]
#[openapi(
    paths(
        // --- Auth ---
        routes::auth::login,

        // --- Admin: User Management ---
        routes::user::create_user,
        routes::user::admin_update_user,
        routes::user::delete_user,
        routes::user::get_all_users,

        // --- User: Self Management ---
        routes::user::self_update_user,

        routes::token::expires,
        
        routes::health::health_check,
    ),
    components(
        schemas(
            // Auth DTOs
            auth::LoginRequest,
            auth::LoginResponse,

            // User DTOs
            user::CreateUserRequest,
            user::UpdateUserRequest,
            user::UpdateProfileRequest,
            user::UserResponse,
            user::UserListResponse,

            token::TokenExpiresResponse,
            
            health::HealthResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication and Tokens"),
        (name = "Admin", description = "System Administration (Users)"),
        (name = "User", description = "User Self-Service"),
        (name = "Token", description = "JWT Token Management")
    ),
    // Attach the Security Scheme (JWT Bearer)
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

// --- Security Configuration ---
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Create the "jwt" security scheme
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            utoipa::openapi::security::SecurityScheme::Http(
                utoipa::openapi::security::Http::new(
                    utoipa::openapi::security::HttpAuthScheme::Bearer,
                ),
            ),
        );
    }
}