use utoipa::OpenApi;
use crate::api::routes;
use crate::models::api::{access, auth, server, user};

#[derive(OpenApi)]
#[openapi(
    paths(
        // --- Auth ---
        routes::auth::login,

        // --- Access (Firewall) ---
        routes::access::request_access,

        // --- Admin: Server Management ---
        routes::server::create_server,
        routes::server::list_servers,
        routes::server::get_server,
        routes::server::update_server,
        routes::server::delete_server,

        // --- Admin: User Management ---
        routes::user::create_user,
        routes::user::admin_update_user,
        routes::user::delete_user,

        // --- User: Self Management ---
        routes::user::self_update_user,
    ),
    components(
        schemas(
            // Auth DTOs
            auth::LoginRequest,
            auth::LoginResponse,

            // Access DTOs
            access::AccessRequest,
            access::AccessResponse,

            // Server DTOs
            server::CreateServerRequest,
            server::UpdateServerRequest,
            server::ServerResponse,

            // User DTOs
            user::CreateUserRequest,
            user::UpdateUserRequest,
            user::UpdateProfileRequest,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication and Tokens"),
        (name = "Access", description = "Firewall and IP Whitelisting"),
        (name = "Admin", description = "System Administration (Servers & Users)"),
        (name = "User", description = "User Self-Service")
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