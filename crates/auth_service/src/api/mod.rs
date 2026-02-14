use crate::api::routes::docs_routes;
use crate::state::AppState;
use axum::{Router, extract::DefaultBodyLimit};
use shared::auth::middleware;

pub mod routes;
pub mod services;
mod docs;

pub fn app(state: AppState) -> Router {
    let users = routes::user_routes();
    let admin = routes::admin_routes();

    let swagger = docs_routes();

    let public_api = Router::new()
        .merge(routes::public_routes());

    let secure_api = Router::new()
        .nest("/admin", admin)
        .nest("/users", users)
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::auth::<AppState>,
        ));

    let aggregated_routes = Router::new()
        .merge(secure_api)
        .merge(public_api)
        .with_state(state);

    Router::new()
        .nest("/api/v1", aggregated_routes)
        .layer(DefaultBodyLimit::max(4096))
        .merge(swagger)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::persistence::repository::Repositories;
    use crate::persistence::repository::interface::user::MockUserRepository;
    use crate::persistence::repository::interface::refresh_token::MockRefreshTokenRepository;
    use axum::body::Body;
    use axum::response::Response;
    use http::{Request, StatusCode};
    use tower::ServiceExt;
    use std::sync::Arc;
    use shared::auth::jwt;
    use shared::auth::models::Claims;
    use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

    fn setup_test_app() -> (Router, String) {
        let mut rng = rand::rng();
        let priv_key = rsa::RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let priv_pem = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap().to_string();
        let pub_pem = priv_key.to_public_key().to_public_key_pem(rsa::pkcs8::LineEnding::LF).unwrap();

        let config = AppConfig {
            private_key_pem: priv_pem.clone(),
            public_key_pem: pub_pem,
            database_path: "dummy".to_string(),
        };

        let repos = Repositories {
            user: Arc::new(MockUserRepository::new()),
            refresh_token: Arc::new(MockRefreshTokenRepository::new()),
        };

        let state = AppState::new(config, repos);
        (app(state), priv_pem)
    }

    #[tokio::test]
    async fn test_public_route_health() {
        let (app, _) = setup_test_app();

        let response = app
            .oneshot(Request::builder().uri("/api/v1/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_secure_route_unauthorized() {
        let (app, _) = setup_test_app();

        let response = app
            .oneshot(Request::builder().uri("/api/v1/users/profile").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_admin_route_forbidden_for_user() {
        let (app, priv_pem) = setup_test_app();

        let claims = Claims::new("testuser".to_string(), false);
        let token = jwt::sign(claims, &priv_pem).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/admin/users")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_admin_route_accessible_by_admin() {
        // We need to set an expectation on the mock because the handler will call it
        // and we are testing the full end-to-end routing here.
        
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_get_all_users().returning(|| Ok(vec![])).times(1);
        
        let mut rng = rand::rng();
        let priv_key = rsa::RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let priv_pem = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap().to_string();
        let pub_pem = priv_key.to_public_key().to_public_key_pem(rsa::pkcs8::LineEnding::LF).unwrap();

        let config = AppConfig {
            private_key_pem: priv_pem.clone(),
            public_key_pem: pub_pem,
            database_path: "dummy".to_string(),
        };

        let repos = Repositories {
            user: Arc::new(user_repo),
            refresh_token: Arc::new(MockRefreshTokenRepository::new()),
        };

        let state = AppState::new(config, repos);
        let router = app(state);

        let claims = Claims::new("admin".to_string(), true);
        let token = jwt::sign(claims, &priv_pem).unwrap();
        
        let request = Request::builder()
            .uri("/api/v1/admin/users")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response: Response = router.oneshot(request).await.unwrap();

        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
        assert_ne!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_body_limit_exceeded() {
        let (app, _) = setup_test_app();

        let large_body = vec![0u8; 5000];
        
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(large_body))
                    .unwrap()
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }
}
