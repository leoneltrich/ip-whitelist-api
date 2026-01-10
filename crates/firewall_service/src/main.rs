use std::sync::Arc;
use crate::config::AppConfig;
use tokio::net::TcpListener;

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

    let pool = initialization::run_startup_sequence(&config.database_path).await?;
    let repositories = Repositories::new(pool);

    let firewall: Arc<dyn FirewallBackend> = match config.firewall_backend.as_str() {
        "nftables" => {
            println!("🛡️ Selected Backend: NFTables (Linux)");
            Arc::new(NftablesFirewall::new())
        },
        "mock" => {
            println!("🛡️ Selected Backend: Mock (Safe Mode)");
            Arc::new(MockFirewall)
        },
        &_ => {
            println!("Invalid firewall backend specified in FIREWALL_BACKEND env var");
            return Err("Invalid firewall backend specified".into());
        }
    };

    firewall.setup().await?;

    let app_state = AppState::new(config, repositories, firewall);

    let app = api::app(app_state);
    let listener = TcpListener::bind("0.0.0.0:3001").await?;
    println!("Firewall Service listening on port 3001");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>()
    ).await?;

    Ok(())
}