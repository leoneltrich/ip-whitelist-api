use tokio::net::TcpListener;
use tracing::info;
use shared::logging::init_logging;
use crate::initialization::run_startup_sequence;
use crate::persistence::repository::Repositories;
use crate::state::AppState;
use crate::state::config::AppConfig;

mod api;
mod initialization;
mod models;
mod persistence;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new();
    
    let _guard = init_logging(&config.log_config);
    info!("Firewall Service starting up...");

    let pool = run_startup_sequence(&config.database_path).await?;
    info!("Database connection established");
    
    let repositories = Repositories::new(pool);
    info!("Repositories initialized");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &config.listen_port)).await?;
    info!("Listening on port {}", &config.listen_port);
    
    let app_state = AppState::new(config, repositories);
    info!("App state initialized");

    let app = api::app(app_state);
    
    info!("Startup complete.");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
        .await?;


    Ok(())
}
