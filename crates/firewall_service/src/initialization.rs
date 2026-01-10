use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::time::Duration;

pub async fn run_startup_sequence(database_path: &str) -> Result<SqlitePool, sqlx::Error> {
    println!("🚀 Starting Firewall Service...");
    println!("📂 Database path: {}", database_path);

    let connection_url = format!("sqlite:{}", database_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&connection_url)
        .await?;

    println!("✅ Database connection established.");

    Ok(pool)
}
