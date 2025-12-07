use tokio::net::TcpListener;

mod persistence;
mod api;
mod model;
mod filesystem;
mod initialization;

use crate::persistence::repository::Repositories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = initialization::run_startup_sequence().await?;

    // 1. Create Repositories
    let repos = Repositories::new(pool);

    // 2. Pass repos to the API
    let app = api::app(repos); // <--- Pass it here

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server listening on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}