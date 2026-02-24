use crate::api::routes;
use crate::models::api::{access, server};
use shared::health::models as health;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::access::request_access,
        routes::access::check_access_status,
        routes::server::create_server,
        routes::server::list_servers,
        routes::server::get_server,
        routes::server::update_server,
        routes::server::delete_server,
        routes::server::check_server_exists,
        shared::health::routes::health_check
    ),
    components(
        schemas(
            access::AccessRequest,
            access::AccessResponse,
            access::AccessStatusResponse,
            server::CreateServerRequest,
            server::UpdateServerRequest,
            server::ServerResponse,
            server::ServerExistsResponse,
            health::HealthResponse,
        )
    ),
    tags(
        (name = "Servers", description = "Manage servers (CRUD)"),
        (name = "Access", description = "Set and get access to servers"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            utoipa::openapi::security::SecurityScheme::Http(utoipa::openapi::security::Http::new(
                utoipa::openapi::security::HttpAuthScheme::Bearer,
            )),
        );
    }
}
