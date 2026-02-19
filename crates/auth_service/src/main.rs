use crate::config::AppConfig;
use tokio::net::TcpListener;
use tracing::info;
use shared::logging::init_logging;

mod api;
mod config;
mod initialization;
mod models;
mod persistence;
mod security;
mod state;

use crate::persistence::repository::Repositories;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new();

    let _guard = init_logging(&config.log_config);
    info!("Auth Service starting up...");

    let pool = initialization::run_startup_sequence(&config.database_path).await?;
    info!("Database connection established");

    let repositories = Repositories::new(pool);
    info!("Repositories initialized");

    let app_state = AppState::new(config, repositories);
    info!("App state initialized");

    let app = api::app(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    info!("Listening on port 3000");
    info!("Startup complete.");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
