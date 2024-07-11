use argon2::{
    self,
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use lazy_static::lazy_static;
use rusqlite::{params, Connection, Result};
use serde_json;
use shared::UserData;
use std::sync::Mutex;

lazy_static! {
    static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(
        Connection::open("/var/www/your_app/backend/users.db").expect("Failed to open database")
    );
}

pub fn init_db() -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            user_data TEXT NOT NULL,
            email TEXT,
            created_at TEXT,
            last_login TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn add_user(username: &str, password: &str, email: &str) -> Result<(), rusqlite::Error> {
    let conn = DB_CONNECTION.lock().unwrap();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let now = Utc::now().to_rfc3339();

    let initial_user_data = UserData::default();

    let user_data_json = serde_json::to_string(&initial_user_data).unwrap();

    conn.execute(
        "INSERT INTO users (username, password, email, created_at, last_login, user_data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![username, &password_hash, email, &now, &now, &user_data_json],
    )?;
    Ok(())
}

pub fn verify_user(username: &str, password: &str) -> Result<Option<UserData>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT password, user_data FROM users WHERE username = ?1")?;
    let mut rows = stmt.query([username])?;

    if let Some(row) = rows.next()? {
        let stored_hash: String = row.get(0)?;
        let user_data_json: String = row.get(1)?;

        // Verify password
        let parsed_hash = PasswordHash::new(&stored_hash).unwrap();
        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        if is_valid {
            // Parse user_data JSON into UserData struct
            let user_data: UserData = serde_json::from_str(&user_data_json).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
            Ok(Some(user_data))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn update_user_data(username: &str, user_data: &UserData) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    let serialized_data = serde_json::to_string(&user_data).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;

    conn.execute(
        "UPDATE users SET user_data = ?1, last_login = ?2 WHERE username = ?3",
        params![&serialized_data, &now, username],
    )?;
    Ok(())
}
