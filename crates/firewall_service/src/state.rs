use crate::config::AppConfig;
use crate::persistence::repository::Repositories;
use crate::system::firewall::FirewallBackend;
use shared::auth::middleware::AuthState;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub repositories: Repositories,
    pub firewall: Arc<dyn FirewallBackend>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        repositories: Repositories,
        firewall: Arc<dyn FirewallBackend>,
    ) -> Self {
        Self {
            config: Arc::new(config),
            repositories,
            firewall,
        }
    }
}

impl AuthState for AppState {
    fn public_key_pem(&self) -> &str {
        &self.config.public_key_pem
    }
}
