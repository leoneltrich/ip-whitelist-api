use std::str::FromStr;
use std::time::Duration;
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
    println!("DEFAULT ADMIN CREATED, CHANGE PASSWORD IMMEDIATELY:");
    println!("Username: {}", admin_username);
    println!("Password: {}", password_plain);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize_creates_tables() {
        // Use in-memory DB for isolated test
        let pool = initialize(":memory:").await.unwrap();

        // Check if users table exists
        let users_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='users'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(users_exists.0, 1);

        // Check if refresh_tokens table exists
        let tokens_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='refresh_tokens'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(tokens_exists.0, 1);
    }

    #[tokio::test]
    async fn test_create_users_table_explicitly() {
        let options = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePool::connect_with(options).await.unwrap();

        create_users_table(&pool).await.unwrap();

        let table_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='users'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(table_exists.0, 1);
    }

    #[tokio::test]
    async fn test_create_refresh_token_table_explicitly() {
        let options = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = SqlitePool::connect_with(options).await.unwrap();
        
        // Users table must exist for FK constraint
        create_users_table(&pool).await.unwrap();
        create_refresh_token_table(&pool).await.unwrap();

        let table_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='refresh_tokens'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(table_exists.0, 1);
    }

    #[tokio::test]
    async fn test_bootstrap_admin_fresh_install() {
        let pool = initialize(":memory:").await.unwrap();
        let repos = Repositories::new(pool);

        // Verify admin doesn't exist
        assert!(repos.user.get_user_by_name("admin").await.unwrap().is_none());

        // Run bootstrap
        bootstrap_admin(&repos).await.unwrap();

        // Verify admin exists
        let admin = repos.user.get_user_by_name("admin").await.unwrap().unwrap();
        assert_eq!(admin.username, "admin");
        assert!(admin.is_admin);
    }

    #[tokio::test]
    async fn test_bootstrap_admin_idempotency() {
        let pool = initialize(":memory:").await.unwrap();
        let repos = Repositories::new(pool);

        // 1. Manually create an admin with a known hash
        let initial_admin = User {
            username: "admin".to_string(),
            password_hash: "existing_hash".to_string(),
            is_admin: true,
        };
        repos.user.create_user(&initial_admin).await.unwrap();

        // 2. Run bootstrap
        bootstrap_admin(&repos).await.unwrap();

        // 3. Verify it didn't overwrite the existing admin
        let admin = repos.user.get_user_by_name("admin").await.unwrap().unwrap();
        assert_eq!(admin.password_hash, "existing_hash");
    }

    #[tokio::test]
    async fn test_run_startup_sequence_full_flow() {
        // This tests the integration of initialize and bootstrap_admin
        let pool = run_startup_sequence(":memory:").await.unwrap();

        // Verify admin was created
        let admin_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM users WHERE username = 'admin'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(admin_exists.0, 1);
    }
}
