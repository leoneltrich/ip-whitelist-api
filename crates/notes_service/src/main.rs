use tokio::net::TcpListener;
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

    let pool = run_startup_sequence(&config.database_path).await?;
    let repositories = Repositories::new(pool);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &config.listen_port)).await?;

    let app_state = AppState::new(config, repositories);

    let app = api::app(app_state);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
        .await?;


    Ok(())
}
