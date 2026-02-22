use crate::health::models::HealthResponse;
use axum::{response::IntoResponse, Json};

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    ),
    tags = ["Health"]
)]
pub async fn health_check(version: &str) -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: version.to_string(),
    };
    Json(response)
}