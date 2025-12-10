// src/persistence/sqlite/mod.rs

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::{Error};
use std::str::FromStr;
use std::time::Duration;

/// Establishes connection pool and runs schema migration
pub async fn initialize(path: &str) -> Result<SqlitePool, Error> {
    // 1. Configure Options
    // create_if_missing(true) is key here.
    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", path))?
        .create_if_missing(true)
        .foreign_keys(true);

    // 2. Create the Pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await?;

    // 3. Run Schema Setup
    // Since we are async now, we can await these directly.
    create_schema(&pool).await?;

    Ok(pool)
}

async fn create_schema(pool: &SqlitePool) -> Result<(), Error> {
    // User Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL DEFAULT 0
        );"
    ).execute(pool).await?;

    // Server Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS servers (
            servername TEXT PRIMARY KEY,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL,
            api_startup_method TEXT NOT NULL,
            api_startup_link TEXT NOT NULL,
            api_startup_token TEXT NOT NULL
        );"
    ).execute(pool).await?;

    // Mapping Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_server_map (
            username TEXT NOT NULL,
            servername TEXT NOT NULL,
            PRIMARY KEY (username, servername),
            FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
            FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE
        );"
    ).execute(pool).await?;

    Ok(())
}