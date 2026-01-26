// src/persistence/sqlite/mod.rs

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Error;
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
    create_users_table(&pool).await?;
    create_refresh_token_table(&pool).await?;

    Ok(pool)
}

async fn create_users_table(pool: &SqlitePool) -> Result<(), Error> {
    // User Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL DEFAULT 0
        );",
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn create_refresh_token_table(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query("CREATE TABLE refresh_tokens (
        token_id TEXT PRIMARY KEY,
        username TEXT NOT NULL,
        token_hash TEXT NOT NULL,
        expires_at INTEGER NOT NULL,
        created_at INTEGER NOT NULL,
        is_revoked BOOLEAN DEFAULT 0,
        FOREIGN KEY(username) REFERENCES users(username) ON DELETE CASCADE
    );
        CREATE INDEX idx_rt_username ON refresh_tokens(username);",
    )
    .execute(pool)
    .await?;
    Ok(())
}
