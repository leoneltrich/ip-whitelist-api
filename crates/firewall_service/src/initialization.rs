use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::Duration;

pub async fn run_startup_sequence(database_path: &str) -> Result<SqlitePool, sqlx::Error> {
    println!("🚀 Starting Firewall Service...");
    println!("📂 Database path: {}", database_path);

    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", database_path))?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await?;

    println!("✅ Database connection established.");

    create_schema(&pool).await?;

    Ok(pool)
}

async fn create_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Server Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS servers (
            servername TEXT PRIMARY KEY,
            port INTEGER NOT NULL,
            api_startup_method TEXT,
            api_startup_link TEXT,
            api_startup_token TEXT
        );"
    ).execute(pool).await?;

    // Whitelist Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS whitelist (
            servername TEXT NOT NULL,
            username TEXT NOT NULL,
            ip_address TEXT NOT NULL,
            expiration INTEGER NOT NULL,

            -- Composite Primary Key
            PRIMARY KEY (servername, username, ip_address),

            -- Foreign Keys
            FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE
        );"
    ).execute(pool).await?;

    // User Server Mapping Table (Permissions)
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_server_map (
            username TEXT NOT NULL,
            servername TEXT NOT NULL,
            PRIMARY KEY (username, servername),
            FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE
        );"
    ).execute(pool).await?;

    println!("✅ Database schema initialized.");
    Ok(())
}
