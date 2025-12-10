use rand::distr::Alphanumeric;
use rand::Rng;
// src/initialization.rs
use crate::persistence::sqlite;
use sqlx::SqlitePool;
use crate::models::database::user::User;
use crate::persistence::repository::Repositories;
use crate::security::hashing;

pub async fn run_startup_sequence(database_path: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    println!("Initializing Persistence Layer...");

    // We await the result here
    let pool = sqlite::initialize(database_path).await?;

    println!("Database setup complete.");

    let repos = Repositories::new(pool.clone());
    bootstrap_admin(&repos).await?;

    Ok(pool)
}

async fn bootstrap_admin(repos: &Repositories) -> Result<(), Box<dyn std::error::Error>> {
    let admin_username = "admin";

    // Check if admin already exists
    if repos.user.get_user(admin_username).await?.is_some() {
        return Ok(()); // Admin exists, nothing to do.
    }

    println!("No admin user found. Creating default admin...");

    // 1. Generate Random Password (32 chars)
    let password_plain: String = rand::thread_rng()
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
    println!("AVE THIS PASSWORD NOW. IT WILL NOT BE SHOWN AGAIN.");
    println!("========================================================\n");

    Ok(())
}