use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::Duration;

pub async fn run_startup_sequence(
    database_path: &str,
) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    println!("Initializing Persistence Layer...");

    let options = SqliteConnectOptions::from_str(&format!("sqlite://{}", database_path))?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(options)
        .await?;

    create_schema(&pool).await?;

    Ok(pool)
}

async fn create_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS notes (
              note_id INTEGER PRIMARY KEY AUTOINCREMENT,
              owner_id TEXT NOT NULL,
              is_public_read BOOLEAN NOT NULL DEFAULT 0,
              is_public_write BOOLEAN NOT NULL DEFAULT 0,
              title TEXT,
              content TEXT NOT NULL,
              timestamp_created INTEGER NOT NULL,
              timestamp_modified INTEGER NOT NULL
          );
          
          -- Index for retrieving a user's own notes efficiently
          CREATE INDEX IF NOT EXISTS idx_notes_owner_date ON notes (owner_id, timestamp_modified DESC);
          
          -- Index for retrieving public notes efficiently
          CREATE INDEX IF NOT EXISTS idx_notes_public_date ON notes (is_public_read, timestamp_modified DESC);
          "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}