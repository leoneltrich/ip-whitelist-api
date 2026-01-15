mod api;
mod config;
mod domain;
mod monitor;

use crate::config::Config;
use crate::domain::SystemHealth;
use crate::monitor::{HealthMonitor, SharedHealthState};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("debug").init();

    info!("Health Service starting up...");

    // 1. Load Config
    let config = match Config::load_from_env().await {
        Ok(c) => c,
        Err(e) => {
            error!("Fatal: {}", e);
            std::process::exit(1);
        }
    };

    let port = config.port; // Copy port before moving state

    // 2. Initialize Shared State
    let state: SharedHealthState = Arc::new(RwLock::new(SystemHealth::new()));

    // 3. Spawn Monitor Task
    let monitor = HealthMonitor::new(config, state.clone());
    tokio::spawn(async move {
        monitor.run().await;
    });

    // 4. Start API Server
    let app = api::router(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Health API listening on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
