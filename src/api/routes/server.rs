use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde_json::json;
use crate::state::AppState;
use crate::models::api::server::{CreateServerRequest, UpdateServerRequest};
use crate::api::services::server as server_service;
use crate::errors::AppError;

// POST /servers
pub async fn create_server(
    State(state): State<AppState>,
    Json(payload): Json<CreateServerRequest>,
) -> Result<impl IntoResponse, AppError> {
    server_service::create_server(&state.repositories, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({"status": "success", "message": "Server created"}))
    ))
}

// GET /servers
pub async fn list_servers(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let servers = server_service::list_servers(&state.repositories).await?;
    Ok(Json(servers))
}

// GET /servers/:name
pub async fn get_server(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let server = server_service::get_server(&state.repositories, name).await?;
    Ok(Json(server))
}

// PUT /servers/:name
pub async fn update_server(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(payload): Json<UpdateServerRequest>,
) -> Result<impl IntoResponse, AppError> {
    server_service::update_server(&state.repositories, name, payload).await?;

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "message": "Server updated"}))
    ))
}

// DELETE /servers/:name
pub async fn delete_server(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    server_service::delete_server(&state.repositories, name).await?;

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "message": "Server deleted"}))
    ))
}