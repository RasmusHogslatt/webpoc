use egui::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::*;
use std::future::Future;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TemplateApp {
    user: User,
    #[serde(skip)]
    client: Client,
    login_status: Option<bool>,
    registration_status: Option<bool>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            user: User::default(),
            login_status: None,
            registration_status: None,
            client: Client::new(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn spawn_task<F: Future<Output = ()> + 'static>(future: F) {
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_task<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(future);
    });
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    async fn verify_user(
        username: String,
        password: String,
        client: Client,
    ) -> Result<Option<UserData>, Box<dyn std::error::Error>> {
        let user = User {
            username,
            password,
            email: None,
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
    async fn update_user_data(
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

    async fn register_user(
        username: String,
        password: String,
        client: Client,
    ) -> Result<bool, reqwest::Error> {
        let user = User {
            username,
            password,
            email: None,
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
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("User Management");

            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.user.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.text_edit_singleline(&mut self.user.password);
            });

            ui.horizontal(|ui| {
                if ui.button("Sign in").clicked() {
                    let username = self.user.username.clone();
                    let password = self.user.password.clone();
                    let client = self.client.clone();
                    let ctx = ctx.clone();

                    spawn_task(async move {
                        match Self::verify_user(username, password, client).await {
                            Ok(Some(user_data)) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp("login_status".into(), true);
                                    mem.data.insert_temp("user_data".into(), user_data);
                                });
                            }
                            Ok(None) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp("login_status".into(), false);
                                });
                            }
                            Err(_) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp("login_status".into(), false);
                                });
                            }
                        }
                    });
                }

                if ui.button("Register").clicked() {
                    let username = self.user.username.clone();
                    let password = self.user.password.clone();
                    let client = self.client.clone();
                    let ctx = ctx.clone();

                    spawn_task(async move {
                        match Self::register_user(username, password, client).await {
                            Ok(is_registered) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data
                                        .insert_temp("registration_status".into(), is_registered);
                                });
                            }
                            Err(e) => {
                                let error_message = format!("Registration failed: {:?}", e);
                                println!("{}", error_message); // This will print to the browser console
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp("registration_status".into(), false);
                                    mem.data.insert_temp("error_message".into(), error_message);
                                });
                            }
                        }
                    });
                }
            });

            ui.heading("User Data");
            let changed = ui
                .color_edit_button_srgba(&mut self.user.user_data.favorite_color)
                .changed();
            if changed {
                let updated_user = self.user.clone();
                let client = self.client.clone();
                let ctx = ctx.clone();

                spawn_task(async move {
                    match Self::update_user_data(updated_user, client).await {
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

            if let Some(status) = ctx.memory(|mem| mem.data.get_temp("login_status".into())) {
                ui.colored_label(
                    if status { Color32::GREEN } else { Color32::RED },
                    if status {
                        "Login successful!"
                    } else {
                        "Login failed!"
                    },
                );
            }

            if let Some(status) = ctx.memory(|mem| mem.data.get_temp("registration_status".into()))
            {
                ui.colored_label(
                    if status { Color32::GREEN } else { Color32::RED },
                    if status {
                        "Registration successful!"
                    } else {
                        "Registration failed!"
                    },
                );
            }

            if let Some(status) = ctx.memory(|mem| mem.data.get_temp("update_status".into())) {
                ui.colored_label(
                    if status { Color32::GREEN } else { Color32::RED },
                    if status {
                        "User data updated successfully!"
                    } else {
                        "Failed to update user data!"
                    },
                );
            }
        });
    }
}
