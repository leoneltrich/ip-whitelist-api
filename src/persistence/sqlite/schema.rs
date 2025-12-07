use rusqlite::{Connection, Result};

/// Creates the database tables if they do not exist.
pub(super) fn create_tables(conn: &Connection) -> Result<()> {
    // 1. Users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;

    // 2. Servers
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servers (
            servername TEXT PRIMARY KEY,
            ip_address TEXT NOT NULL,
            port INTEGER NOT NULL,
            api_startup_method TEXT NOT NULL,
            api_startup_link TEXT NOT NULL,
            api_startup_token TEXT NOT NULL
        )",
        [],
    )?;

    // 3. Mapping (N:M)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_server_map (
            username TEXT NOT NULL,
            servername TEXT NOT NULL,
            PRIMARY KEY (username, servername),
            FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE,
            FOREIGN KEY (servername) REFERENCES servers(servername) ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(())
}