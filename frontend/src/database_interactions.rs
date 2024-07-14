use crate::app::Application;
use egui::Context;
use reqwest::Client;
use shared::{User, UserData};
use std::future::Future;

#[cfg(target_arch = "wasm32")]
pub fn spawn_task<F: Future<Output = ()> + 'static>(future: F) {
    wasm_bindgen_futures::spawn_local(future);
}

impl Application {
    pub async fn verify_user(
        username: String,
        password: String,
        client: Client,
    ) -> Result<Option<UserData>, Box<dyn std::error::Error>> {
        let user = User {
            username,
            password,
            email: "".to_string(),
            created_at: None,
            last_login: None,
            user_data: UserData::default(),
        };
        let response = client
            .post("http://138.68.94.119/api/login")
            .json(&user)
            .send()
            .await?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await?;
            println!("Login response JSON: {:?}", json); // Debug log

            if json["status"] == "success" {
                let user_data: UserData = serde_json::from_value(json["user_data"].clone())?;
                println!("Parsed user data: {:?}", user_data); // Debug log
                Ok(Some(user_data))
            } else {
                println!("Login failed with status: {:?}", json["status"]); // Debug log
                Ok(None)
            }
        } else {
            println!("Login failed with status code: {:?}", response.status()); // Debug log
            Ok(None)
        }
    }
    pub async fn update_user_data(
        user: User,
        client: Client,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let response = client
            .post("http://138.68.94.119/api/update_user_data")
            .json(&user) // send the entire User object
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

    pub async fn register_user(
        username: String,
        password: String,
        email: String,
        client: Client,
    ) -> Result<bool, reqwest::Error> {
        let user = User {
            username,
            password,
            email,
            created_at: None,
            last_login: None,
            user_data: UserData::default(),
        };
        let response = client
            .post("http://138.68.94.119/api/register")
            .json(&user)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    pub fn save_to_database(&self, ctx: &Context) {
        let user = self.user.clone();
        let client = self.client.clone();
        let ctx = ctx.clone();

        crate::app::spawn_task(async move {
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
