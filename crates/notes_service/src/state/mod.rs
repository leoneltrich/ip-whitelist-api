pub(crate) mod config;

use crate::persistence::repository::Repositories;
use shared::auth::middleware::AuthState;
use std::sync::Arc;
use crate::state::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub repositories: Repositories,
}

impl AppState {
    pub fn new(config: AppConfig, repositories: Repositories) -> Self {
        Self {
            config: Arc::new(config),
            repositories,
        }
    }
}

impl AuthState for AppState {
    fn public_key_pem(&self) -> &str {
        &self.config.public_key_pem
    }
}
