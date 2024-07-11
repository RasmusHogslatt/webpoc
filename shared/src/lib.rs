use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub username: String,
    pub favorite_color: String,
    pub age: u32,
    pub email: Option<String>,
    pub created_at: Option<String>, // You could use chrono::DateTime<Utc> if you add the chrono crate
    pub last_login: Option<String>, // Same as above
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
