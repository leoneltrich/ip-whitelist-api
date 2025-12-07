// src/initialization.rs
use crate::persistence::sqlite;
use sqlx::SqlitePool;

pub async fn run_startup_sequence() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    println!("Initializing Persistence Layer...");

    let db_path = "application.db";

    // We await the result here
    let pool = sqlite::initialize(db_path).await?;

    println!("Database setup complete.");

    Ok(pool)
}