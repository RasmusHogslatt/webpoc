use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
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
            user_data TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn add_user(username: &str, password: &str) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO users (username, password, user_data) VALUES (?1, ?2, ?3)",
        [username, password, "{}"],
    )?;
    Ok(())
}

pub fn verify_user(username: &str, password: &str) -> Result<Option<String>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt =
        conn.prepare("SELECT user_data FROM users WHERE username = ?1 AND password = ?2")?;
    let mut rows = stmt.query([username, password])?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn update_user_data(username: &str, user_data: &UserData) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    let serialized_data = serde_json::to_string(user_data)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    conn.execute(
        "UPDATE users SET user_data = ?1 WHERE username = ?2",
        [&serialized_data, username],
    )?;
    Ok(())
}
