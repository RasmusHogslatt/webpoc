use argon2::{self, Config};
use chrono::Utc;
use egui::Color32;
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
pub fn add_user(
    username: &str,
    password: &str,
    email: Option<&str>,
) -> Result<(), rusqlite::Error> {
    let conn = DB_CONNECTION.lock().unwrap();
    let salt = b"randomsalt"; // In production, use a proper random salt
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), salt, &config).unwrap();

    let now = Utc::now().to_rfc3339();

    let initial_user_data = UserData {
        favorite_color: egui::Color32::BLUE,
    };

    let user_data_json = serde_json::to_string(&initial_user_data).unwrap();

    conn.execute(
        "INSERT INTO users (username, password, email, created_at, last_login, user_data) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![username, &hash, email, &now, &now, &user_data_json],
    )?;
    Ok(())
}

pub fn verify_user(username: &str, password: &str) -> Result<Option<UserData>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT password FROM users WHERE username = ?1")?;
    let mut rows = stmt.query([username])?;

    if let Some(row) = rows.next()? {
        let stored_hash: String = row.get(0)?;
        let user_data_json: String = row.get(1)?;

        // Verify password
        let is_valid = argon2::verify_encoded(&stored_hash, password.as_bytes()).unwrap_or(false);

        if is_valid {
            // Parse user_data JSON into UserData struct
            let user_data: UserData = serde_json::from_str(&user_data_json)?;
            Ok(Some(user_data))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub fn update_user_data(username: &str, user_data: &UserData) -> Result<()> {
    // let conn = DB_CONNECTION.lock().unwrap();
    // let now = Utc::now().to_rfc3339();

    // let mut updated_user_data = user_data.clone();
    // updated_user_data.last_login = Some(now.clone());

    // let serialized_data = serde_json::to_string(&updated_user_data)
    //     .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    // conn.execute(
    //     "UPDATE users SET user_data = ?1, email = ?2, last_login = ?3 WHERE username = ?4",
    //     params![&serialized_data, &updated_user_data.email, &now, username],
    // )?;
    Ok(())
}
