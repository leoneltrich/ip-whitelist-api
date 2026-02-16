use crate::auth::logic;
use crate::auth::models::Claims;
use crate::errors::app_errors::AppError;
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
    Extension,
};

/// A trait that must be implemented by the AppState of any service
/// that wishes to use the generic `auth` middleware.
pub trait AuthState {
    fn public_key_pem(&self) -> &str;
}

/// A generic, shared authentication middleware.
/// It works with any `AppState` type `S` as long as it implements the `AuthState` trait.
pub async fn auth<S>(
    State(state): State<S>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError>
where
    S: AuthState + Clone + Send + Sync + 'static,
{
    let auth_header_value = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::InvalidAccessToken)?;

    let claims = logic::verify_token_from_header(auth_header_value, state.public_key_pem())?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Shared middleware to require admin privileges.
/// It is dependent on the `auth` middleware having run first to insert the claims.
pub async fn require_admin(
    Extension(claims): Extension<Claims>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if !claims.is_admin {
        return Err(AppError::PermissionDenied);
    }
    Ok(next.run(request).await)
}
