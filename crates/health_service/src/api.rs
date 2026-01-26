use crate::domain::{HealthStatus, ServiceHealth, SystemHealth};
use crate::monitor::SharedHealthState;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
    let aggregated_routes = Router::new()
        .route("/health", get(health_check))
        .route("/health/services", get(get_all_services))
        .with_state(state);

    Router::new()
        .nest("/api/v1", aggregated_routes)
        .merge(docs_routes())
}

pub(crate) fn docs_routes() -> Router {
    Router::new().merge(
        SwaggerUi::new("/api/v1/swagger-ui")
            .url("/api/v1/api-docs/openapi.json", ApiDoc::openapi()),
    )
}

/// Quick system health check.
/// Returns 200 if operational, 503 if major failure.
#[utoipa::path(
    get,
    path = "/api/v1/health",
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
    path = "/api/v1/health/services",
    responses(
        (status = 200, description = "List of all services", body = SystemHealth)
    )
)]
async fn get_all_services(State(state): State<SharedHealthState>) -> Json<SystemHealth> {
    let read_guard = state.read().await;
    Json(read_guard.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tower::ServiceExt;
    // for `oneshot`

    #[tokio::test]
    async fn test_health_check_returns_200_when_status_is_up() {
        // Arrange
        let mut health = SystemHealth::new();
        health.status = HealthStatus::Up;
        let state = Arc::new(RwLock::new(health));

        let app = router(state);

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_check_returns_503_when_status_is_down() {
        // Arrange
        let mut health = SystemHealth::new();
        health.status = HealthStatus::Down;
        let state = Arc::new(RwLock::new(health));

        let app = router(state);

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn test_health_check_returns_200_when_status_is_starting() {
        // Arrange
        let mut health = SystemHealth::new();
        health.status = HealthStatus::Starting;
        let state = Arc::new(RwLock::new(health));

        let app = router(state);

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }
}
