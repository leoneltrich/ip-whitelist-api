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
            port INTEGER NOT NULL,
            api_startup_method TEXT, -- No longer 'NOT NULL'
            api_startup_link TEXT,   -- No longer 'NOT NULL'
            api_startup_token TEXT   -- No longer 'NOT NULL'
            -- ip_address is removed
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

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS whitelist (
            servername TEXT NOT NULL,
            username TEXT NOT NULL,
            ip_address TEXT NOT NULL,
            expiration INTEGER NOT NULL,

            -- Composite Primary Key
            PRIMARY KEY (servername, username, ip_address),

            -- Foreign Keys
            FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE,
            FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
        );"
    ).execute(pool).await?;

    Ok(())
}