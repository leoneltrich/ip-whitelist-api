use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use serde_json::json;
use crate::state::AppState;
use crate::models::api::server::{CreateServerRequest, ServerExistsResponse, ServerResponse, UpdateServerRequest};
use crate::api::services::server as server_service;
use crate::errors::AppError;

#[utoipa::path(
    post,
    path = "/admin/servers",
    request_body = CreateServerRequest,
    responses(
        (status = 201, description = "Server created"),
        (status = 409, description = "Server name already exists"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden (Non-admins)")
    ),
    security(("jwt" = []))
)]
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

#[utoipa::path(
    get,
    path = "/admin/servers",
    responses(
        (status = 200, description = "List of servers", body = [ServerResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
pub async fn list_servers(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let servers = server_service::list_servers(&state.repositories).await?;
    Ok(Json(servers))
}

#[utoipa::path(
    get,
    path = "/admin/servers/{name}",
    params(
        ("name" = String, Path, description = "Name of the server")
    ),
    responses(
        (status = 200, description = "Server details", body = ServerResponse),
        (status = 404, description = "Server not found"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
pub async fn get_server(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let server = server_service::get_server(&state.repositories, name).await?;
    Ok(Json(server))
}

#[utoipa::path(
    put,
    path = "/admin/servers/{name}",
    params(
        ("name" = String, Path, description = "Name of the server to update")
    ),
    request_body = UpdateServerRequest,
    responses(
        (status = 200, description = "Server updated"),
        (status = 404, description = "Server not found"),
        (status = 409, description = "New server name conflict"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
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

#[utoipa::path(
    delete,
    path = "/admin/servers/{name}",
    params(
        ("name" = String, Path, description = "Name of the server to delete")
    ),
    responses(
        (status = 200, description = "Server deleted"),
        (status = 404, description = "Server not found"),
        (status = 403, description = "Forbidden")
    ),
    security(("jwt" = []))
)]
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

#[utoipa::path(
    get,
    path = "/users/servers/{name}/exists",
    params(
        ("name" = String, Path, description = "Server name to check")
    ),
    responses(
        (status = 200, description = "Check result", body = ServerExistsResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(("jwt" = []))
)]
pub async fn check_server_exists(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<ServerExistsResponse>, AppError> {

    // We reuse the existing repository function
    let exists = state.repositories.server.get_server_by_name(&name).await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
        .is_some();

    Ok(Json(ServerExistsResponse { exists }))
}