use std::str::FromStr;
use std::time::Duration;
// src/initialization.rs
use crate::models::database::user::User;
use crate::persistence::repository::Repositories;
use crate::security::hashing;
use rand::distr::Alphanumeric;
use rand::RngExt;
use sqlx::{Error, SqlitePool};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

pub async fn run_startup_sequence(
    database_path: &str,
) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    println!("Initializing Persistence Layer...");

    // We await the result here
    let pool = initialize(database_path).await?;

    println!("Database setup complete.");

    let repos = Repositories::new(pool.clone());
    bootstrap_admin(&repos).await?;

    Ok(pool)
}

async fn bootstrap_admin(repos: &Repositories) -> Result<(), Box<dyn std::error::Error>> {
    let admin_username = "admin";

    // Check if admin already exists
    if repos.user.get_user_by_name(admin_username).await?.is_some() {
        println!("Admin user already exists. Skipping creation...");
        return Ok(()); // Admin exists, nothing to do.
    }

    println!("No admin user found. Creating default admin...");

    // 1. Generate Random Password (32 chars)
    let password_plain: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // 2. Hash the password (Never store plain text!)
    let password_hash = hashing::hash_password(&password_plain)
        .map_err(|_| "Password hashing failed".to_string())?;

    // 3. Create the User Model
    let admin_user = User {
        username: admin_username.to_string(),
        password_hash,
        is_admin: true,
    };

    // 4. Save to DB
    repos.user.create_user(&admin_user).await?;

    // 5. Print Credentials to Terminal (Critical Step)
    println!("\n========================================================");
    println!("DEFAULT ADMIN CREATED");
    println!("Username: {}", admin_username);
    println!("Password: {}", password_plain);
    println!("SAVE THIS PASSWORD NOW. IT WILL NOT BE SHOWN AGAIN.");
    println!("========================================================\n");

    Ok(())
}

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
    sqlx::query("CREATE TABLE IF NOT EXISTS refresh_tokens (
        token_hash TEXT NOT NULL PRIMARY KEY,
        username TEXT NOT NULL,
        expires_at INTEGER NOT NULL,
        created_at INTEGER NOT NULL,
        is_revoked BOOLEAN DEFAULT 0,
        FOREIGN KEY(username) REFERENCES users(username) ON DELETE CASCADE
    );
        CREATE INDEX IF NOT EXISTS idx_rt_username ON refresh_tokens(username);",
    )
    .execute(pool)
    .await?;
    Ok(())
}