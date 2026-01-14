use crate::config::AppConfig;
use tokio::net::TcpListener;

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

    let pool = initialization::run_startup_sequence(&config.database_path).await?;
    let repositories = Repositories::new(pool);

    let app_state = AppState::new(config, repositories);

    let app = api::app(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Auth Service listening on port 3000");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
