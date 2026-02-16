use crate::api::routes;
use crate::models::api::{user, auth};
use shared::errors::utoipa_errors as err;
use shared::health::models as health;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // --- Auth ---
        routes::auth::login,
        routes::auth::logout,

        // --- Admin: User Management ---
        routes::user::create_user,
        routes::user::admin_update_user,
        routes::user::delete_user,
        routes::user::get_all_users,

        // --- User: Self Management ---
        routes::user::self_update_user,

        // --- Health check ---
        shared::health::routes::health_check,

        routes::token::refresh
    ),
    components(
        schemas(
            // Auth DTOs
            auth::LoginRequest,
            auth::LoginResponse,
            auth::LogoutRequest,
            auth::LogoutResponse,
            auth::TokenRefreshRequest,
            auth::TokenRefreshResponse,

            // User DTOs
            user::CreateUserRequest,
            user::UpdateUserRequest,
            user::UpdateProfileRequest,
            user::UserResponse,
            user::UserListResponse,

            health::HealthResponse,

            // Errors
            err::InternalServerErrorResponse,
            err::NotFoundErrorResponse,
            err::ConflictErrorResponse,
            err::BadRequestErrorResponse,
            err::PermissionErrorResponse,
            err::LoginAuthErrorResponse,
            err::TokenRefreshErrorResponse,
            err::AccessAuthErrorResponse,
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
            utoipa::openapi::security::SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}
