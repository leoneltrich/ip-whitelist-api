// src/persistence/repository/implementation/user_repository.rs

use rusqlite::{Connection, Result, OptionalExtension};
use crate::model::user::User; // <-- Import the data model
use crate::persistence::repository::interface::user_repository::UserRepository;

pub struct SqliteUserRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> UserRepository for SqliteUserRepository<'a> {
    fn get_user_by_name(&self, username: &str) -> Result<Option<User>> {
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
}