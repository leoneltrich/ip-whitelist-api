use axum::Json;
use serde_json::{json, Value};

// Import the service logic
use crate::api::services::health as health_service;

pub async fn health_handler() -> Json<Value> {
    // Call the business logic
    let message = health_service::get_health_message();

    // Format the HTTP response
    Json(json!({
        "status": message,
        "version": "1.0.0"
    }))
}