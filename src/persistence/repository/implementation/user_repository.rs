// src/persistence/repository/implementation/user_repository.rs

use rusqlite::{Connection, Result, OptionalExtension};
use crate::model::user::User; // <-- Import the data model
use crate::persistence::repository::interface::user_repository::UserRepository;

pub struct SqliteUserRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> UserRepository for SqliteUserRepository<'a> {
    fn get_user(&self, username: &str) -> Result<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT username, password_hash FROM users WHERE username = ?1"
        )?;

        stmt.query_row([username], |row| {
            Ok(User {
                username: row.get(0)?,
                password_hash: row.get(1)?,
            })
        })
            .optional()
    }

    fn update_user(&self, user: &User) -> Result<usize> {
        self.conn.execute(
            "UPDATE users
             SET password_hash = ?1
             WHERE username = ?2",
            [&user.password_hash, &user.username],
        )
    }
    fn delete_user(&self, username: &str) -> Result<usize> {
        self.conn.execute(
            "DELETE FROM users WHERE username = ?1",
            [username],
        )
    }

    fn create_user(&self, user: &User) -> Result<usize> {
        self.conn.execute(
            "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
            [&user.username, &user.password_hash],
        )
    }
}