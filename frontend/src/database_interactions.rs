// database_interactions.rs

use crate::app::Application;
use egui::Context;
use reqwest::Client;
use shared::{User, UserData};
use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Once;

#[cfg(not(target_arch = "wasm32"))]
static INIT: Once = Once::new();

#[cfg(target_arch = "wasm32")]
pub fn spawn_task<F: Future<Output = ()> + 'static>(future: F) {
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_task<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(future);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn init_local_db() -> SqliteResult<()> {
    INIT.call_once(|| {
        let conn = Connection::open("local_users.db").expect("Failed to open database");
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
        )
        .expect("Failed to create users table");
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_db_connection() -> Connection {
    Connection::open("local_users.db").expect("Failed to open database")
}

impl Application {
    pub async fn verify_user(
        username: String,
        password: String,
        client: Client,
    ) -> Result<Option<UserData>, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use local SQLite database for verification
            let conn = get_db_connection();
            let mut stmt =
                conn.prepare("SELECT user_data FROM users WHERE username = ? AND password = ?")?;
            let user_data: Option<String> = stmt
                .query_row([&username, &password], |row| row.get(0))
                .optional()?;

            if let Some(user_data) = user_data {
                let user_data: UserData = serde_json::from_str(&user_data)?;
                Ok(Some(user_data))
            } else {
                Ok(None)
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let user = User {
                username,
                password,
                email: "".to_string(),
                created_at: None,
                last_login: None,
                user_data: UserData::default(),
            };
            let response = client
                .post("https://api.rasmushogslatt.com/api/login")
                .json(&user)
                .send()
                .await?;

            if response.status().is_success() {
                let json: serde_json::Value = response.json().await?;
                println!("Login response JSON: {:?}", json);

                if json["status"] == "success" {
                    let user_data: UserData = serde_json::from_value(json["user_data"].clone())?;
                    println!("Parsed user data: {:?}", user_data);
                    Ok(Some(user_data))
                } else {
                    println!("Login failed with status: {:?}", json["status"]);
                    Ok(None)
                }
            } else {
                println!("Login failed with status code: {:?}", response.status());
                Ok(None)
            }
        }
    }

    pub async fn update_user_data(
        user: User,
        client: Client,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use local SQLite database for updating user data
            let conn = get_db_connection();
            let user_data_json = serde_json::to_string(&user.user_data)?;
            conn.execute(
                "UPDATE users SET user_data = ?, last_login = datetime('now') WHERE username = ?",
                [&user_data_json, &user.username],
            )?;
            Ok(true)
        }

        #[cfg(target_arch = "wasm32")]
        {
            let response = client
                .post("https://api.rasmushogslatt.com/api/update_user_data")
                .json(&user)
                .send()
                .await?;

            if response.status().is_success() {
                Ok(true)
            } else {
                let error_message = response.text().await?;
                println!("Failed to update user data: {:?}", error_message);
                Ok(false)
            }
        }
    }

    pub async fn register_user(
        username: String,
        password: String,
        email: String,
        client: Client,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Use local SQLite database for user registration
            let conn = get_db_connection();
            let user_data = UserData::default();
            let user_data_json = serde_json::to_string(&user_data)?;
            conn.execute(
                "INSERT INTO users (username, password, email, user_data, created_at, last_login) 
                 VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))",
                [&username, &password, &email, &user_data_json],
            )?;
            Ok(true)
        }

        #[cfg(target_arch = "wasm32")]
        {
            let user = User {
                username,
                password,
                email,
                created_at: None,
                last_login: None,
                user_data: UserData::default(),
            };
            let response = client
                .post("https://api.rasmushogslatt.com/api/register")
                .json(&user)
                .send()
                .await?;

            Ok(response.status().is_success())
        }
    }

    pub fn save_to_database(&self, ctx: &Context) {
        let user = self.user.clone();
        let client = self.client.clone();
        let ctx = ctx.clone();

        spawn_task(async move {
            match Application::update_user_data(user, client).await {
                Ok(true) => {
                    ctx.request_repaint();
                    ctx.memory_mut(|mem| {
                        mem.data.insert_temp("update_status".into(), true);
                    });
                }
                _ => {
                    ctx.request_repaint();
                    ctx.memory_mut(|mem| {
                        mem.data.insert_temp("update_status".into(), false);
                    });
                }
            }
        });
    }
}
