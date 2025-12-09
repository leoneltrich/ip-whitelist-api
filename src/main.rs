use tokio::net::TcpListener;
use crate::config::AppConfig;

mod persistence;
mod api;
mod models;
mod filesystem;
mod initialization;
mod errors;
mod state;
mod config;

use crate::persistence::repository::Repositories;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::new();

    let pool = initialization::run_startup_sequence().await?;
    let repositories = Repositories::new(pool);

    let app_state = AppState::new(config, repositories);

    let app = api::app(app_state); // <--- Pass it here
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}