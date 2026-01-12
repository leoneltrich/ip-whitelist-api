use axum::{
    Json,
    Router,
    routing::get,
    extract::State,
    http::StatusCode,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::monitor::SharedHealthState;
use crate::domain::{SystemHealth, ServiceHealth, HealthStatus};

#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_all_services,
    ),
    components(
        schemas(SystemHealth, ServiceHealth, HealthStatus)
    ),
    tags(
        (name = "health", description = "Health Monitoring Endpoints")
    )
)]
pub struct ApiDoc;

pub fn router(state: SharedHealthState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/health/services", get(get_all_services))
        .with_state(state)
}

/// Quick system health check.
/// Returns 200 if operational, 503 if major failure.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "System Operational", body = SystemHealth),
        (status = 503, description = "System Unavailable", body = SystemHealth)
    )
)]
async fn health_check(State(state): State<SharedHealthState>) -> (StatusCode, Json<SystemHealth>) {
    let read_guard = state.read().await;
    let status = match read_guard.status {
        HealthStatus::Down => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::OK,
    };
    (status, Json(read_guard.clone()))
}

/// Get detailed status of all monitored services.
#[utoipa::path(
    get,
    path = "/health/services",
    responses(
        (status = 200, description = "List of all services", body = SystemHealth)
    )
)]
async fn get_all_services(State(state): State<SharedHealthState>) -> Json<SystemHealth> {
    let read_guard = state.read().await;
    Json(read_guard.clone())
}
