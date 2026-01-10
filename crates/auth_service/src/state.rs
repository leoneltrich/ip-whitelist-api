use crate::config::AppConfig;
use crate::persistence::repository::Repositories;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub repositories: Repositories,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        repositories: Repositories,
    ) -> Self {
        Self {
            config: Arc::new(config),
            repositories,
        }
    }
}