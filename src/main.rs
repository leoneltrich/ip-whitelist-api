use std::sync::Arc;
use crate::config::AppConfig;
use tokio::net::TcpListener;

mod api;
mod config;
mod errors;
mod initialization;
mod models;
mod persistence;
mod security;
mod state;
mod system;

use crate::persistence::repository::Repositories;
use crate::state::AppState;
use crate::system::firewall::FirewallBackend;
use crate::system::firewall::mock::MockFirewall;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new();

    let pool = initialization::run_startup_sequence().await?;
    let repositories = Repositories::new(pool);

    let firewall = Arc::new(MockFirewall);

    println!("🔥 Verifying firewall configuration...");
    firewall.validate_config().await.map_err(|e| {
        format!("Firewall validation failed: {:?}", e)
    })?;
    println!("✅ Firewall configured successfully.");

    let app_state = AppState::new(config, repositories, firewall);

    let app = api::app(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>()
    ).await?;

    Ok(())
}
