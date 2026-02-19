use crate::config::AppConfig;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info};
use shared::logging::init_logging;

mod api;
mod config;
mod initialization;
mod models;
mod persistence;
mod state;
mod system;

use crate::persistence::repository::Repositories;
use crate::state::AppState;
use crate::system::firewall::FirewallBackend;
use crate::system::firewall::mock::MockFirewall;
use crate::system::firewall::nftables::NftablesFirewall;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new();

    let _guard = init_logging(&config.log_config);
    info!("Firewall Service starting up...");

    let pool = initialization::run_startup_sequence(&config.database_path).await?;
    info!("Database connection established");

    let repositories = Repositories::new(pool);
    info!("Repositories initialized");

    let firewall: Arc<dyn FirewallBackend> = match config.firewall_backend.as_str() {
        "nftables" => {
            info!("Selected Backend: Nftables (Recommended for production use)");
            Arc::new(NftablesFirewall::new())
        }
        "mock" => {
            info!("Selected Backend: Mock (For testing purposes only)");
            Arc::new(MockFirewall)
        }
        &_ => {
            error!("Invalid firewall backend specified. Available options: nftables, mock.");
            return Err("Invalid firewall backend specified".into());
        }
    };

    firewall.setup().await?;
    info!("Firewall backend initialized");

    let app_state = AppState::new(config, repositories, firewall);
    info!("App state initialized");

    let app = api::app(app_state);
    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    info!("Listening on port 3001");
    
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
