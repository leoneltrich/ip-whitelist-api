// Declare the sibling module
mod schema;

use rusqlite::{Connection, Result};
use std::path::Path;

pub struct SqliteClient {
    pub conn: Connection,
}

/// Establishes connection and runs migration/schema creation
pub fn initialize<P: AsRef<Path>>(path: P) -> Result<SqliteClient> {
    let conn = Connection::open(path)?;

    // Enable Foreign Keys (Critical for your N:M constraints)
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Delegate schema creation to the schema module
    schema::create_tables(&conn)?;

    Ok(SqliteClient { conn })
}