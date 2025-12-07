// src/initialization.rs
use crate::persistence::sqlite;

pub fn run_startup_sequence() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️  Initializing Persistence Layer...");

    // We call the sqlite module which now handles the schema internally
    let _db_client = sqlite::initialize("application.db")?;

    println!("✅ Database setup complete.");
    Ok(())
}