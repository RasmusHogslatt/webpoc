use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

lazy_static! {
    static ref DB_CONNECTION: Mutex<Connection> =
        Mutex::new(Connection::open("users.db").expect("Failed to open database"));
}

pub fn init_db() -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn add_user(username: &str, password: &str) -> Result<()> {
    let conn = DB_CONNECTION.lock().unwrap();
    conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        [username, password],
    )?;
    Ok(())
}

pub fn verify_user(username: &str, password: &str) -> Result<bool> {
    let conn = DB_CONNECTION.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?1 AND password = ?2")?;
    let rows = stmt.query_map([username, password], |_| Ok(()))?;
    Ok(rows.count() > 0)
}
