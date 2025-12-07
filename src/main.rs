use std::sync::Arc;
use tokio::net::TcpListener;

mod persistence;
mod api;
mod model;
mod filesystem;
mod initialization;

use crate::persistence::repository::Repositories;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Init the DB connection (returns SqlitePool)
    let pool = initialization::run_startup_sequence().await?;

    // 2. Create the Repositories
    // The "new" method inside persistence handles the dirty work of
    // creating the concrete types and wrapping them in Arc.
    let repos = Repositories::new(pool);

    let app = api::app();
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("🚀 Server listening on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}