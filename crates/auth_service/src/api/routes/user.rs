use crate::api::services::{user as user_service, user};
use crate::models::api::user::{
    CreateUserRequest, UpdateProfileRequest, UpdateUserRequest, UserListResponse,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State}, http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use serde_json::json;
use shared::auth::models::Claims;
use shared::errors::app_errors::AppError;
use shared::errors::utoipa_errors::{AccessAuthErrorResponse, BadRequestErrorResponse, ConflictErrorResponse, InternalServerErrorResponse, NotFoundErrorResponse, PermissionErrorResponse};
// Self-routes

#[utoipa::path(
    put,
    path = "/api/v1/users/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = [])),
    tags = ["User"]
)]
pub async fn self_update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, AppError> {
    let trusted_request = user::sanitize_user_self_update_request(&state, &claims, payload).await?;

    user_service::update_user(&state, trusted_request).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "Profile updated successfully"
        })),
    ))
}

// Admin routes

#[utoipa::path(
    post,
    path = "/api/v1/admin/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 409, description = "Username already exists", body = ConflictErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = [])),
    tags = ["Admin"]
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    user_service::create_user(&state, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "created",
            "message": "User successfully created"
        })),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/admin/users",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "User not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = [])),
    tags = ["Admin"]
)]
pub async fn admin_update_user(
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    user_service::update_user(&state, payload).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "User data updated successfully"
        })),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/admin/users/{username}",
    params(
        ("username" = String, Path, description = "Username to delete")
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "User not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = [])),
    tags = ["Admin"]
)]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    user_service::delete_user(&state, username).await?;

    Ok((
        StatusCode::NO_CONTENT,
        Json(json!({
            "status": "deleted",
            "message": "User successfully deleted"
        })),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    responses(
        (status = 200, description = "List of all users retrieved successfully", body = UserListResponse),
        (status = 400, description = "Invalid request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthenticated", body = AccessAuthErrorResponse),
        (status = 403, description = "Unauthorized", body = PermissionErrorResponse),
        (status = 404, description = "Resource not found", body = NotFoundErrorResponse),
        (status = 500, description = "An internal server error occurred", body = InternalServerErrorResponse)
    ),
    security(("jwt" = [])),
    tags = ["Admin"]
)]
pub async fn get_all_users(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let users = user_service::get_all_users(&state).await?;

    let response = UserListResponse {
        status: "success".to_string(),
        data: users,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::models::database::user::User;
    use crate::persistence::repository::interface::refresh_token::MockRefreshTokenRepository;
    use crate::persistence::repository::interface::user::MockUserRepository;
    use crate::persistence::repository::Repositories;
    use axum::extract::State;
    use axum::{Extension, Json};
    use std::sync::Arc;
    use shared::logging::models::LogConfig;

    fn setup_test_state(user_repo: MockUserRepository) -> AppState {
        let repositories = Repositories {
            user: Arc::new(user_repo),
            refresh_token: Arc::new(MockRefreshTokenRepository::new()),
        };
        let config = AppConfig {
            private_key_pem: "dummy".to_string(),
            public_key_pem: "dummy".to_string(),
            database_path: "dummy".to_string(),
            log_config: LogConfig::new_dummy(),
        };
        AppState::new(config, repositories)
    }

    #[tokio::test]
    async fn test_self_update_user_prevents_admin_escalation() {
        let mut user_repo = MockUserRepository::new();

        // 1. Database says the user is NOT an admin
        user_repo.expect_get_user_by_name().returning(|_| {
            Ok(Some(User {
                username: "attacker".into(),
                password_hash: "hash".into(),
                is_admin: false, // The ground truth
            }))
        });

        // 2. Verification: The subsequent update MUST be called with is_admin: false
        user_repo
            .expect_update_user()
            .withf(|user_to_update| user_to_update.is_admin == false)
            .returning(|_| Ok(1))
            .times(1);

        let state = setup_test_state(user_repo);

        // 3. The incoming JWT claims incorrectly (or maliciously) say is_admin: true
        let claims = Claims::new("attacker".into(), true);
        let payload = UpdateProfileRequest {
            password: "new_password".into(),
        };

        let result = self_update_user(State(state), Extension(claims), Json(payload)).await;

        assert!(result.is_ok());
    }
}
