use egui::Color32;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::User;
use std::future::Future;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TemplateApp {
    username: String,
    password: String,
    login_status: Option<bool>,
    registration_status: Option<bool>,
    #[serde(skip)]
    client: Client,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
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
    ) -> Result<bool, reqwest::Error> {
        let user = User { username, password };
        let response = client
            .post("http://138.68.94.119/api/login")
            .json(&user)
            .send()
            .await?;

        Ok(response.status().is_success())
    }

    async fn register_user(
        username: String,
        password: String,
        client: Client,
    ) -> Result<bool, reqwest::Error> {
        let user = User { username, password };
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
                ui.text_edit_singleline(&mut self.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.text_edit_singleline(&mut self.password);
            });

            ui.horizontal(|ui| {
                if ui.button("Sign in").clicked() {
                    let username = self.username.clone();
                    let password = self.password.clone();
                    let client = self.client.clone();
                    let ctx = ctx.clone();

                    spawn_task(async move {
                        match Self::verify_user(username, password, client).await {
                            Ok(is_valid) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp("login_status".into(), is_valid)
                                });
                            }
                            Err(_) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data.insert_temp("login_status".into(), false)
                                });
                            }
                        }
                    });
                }

                if ui.button("Register").clicked() {
                    let username = self.username.clone();
                    let password = self.password.clone();
                    let client = self.client.clone();
                    let ctx = ctx.clone();

                    spawn_task(async move {
                        match Self::register_user(username, password, client).await {
                            Ok(is_registered) => {
                                ctx.request_repaint();
                                ctx.memory_mut(|mem| {
                                    mem.data
                                        .insert_temp("registration_status".into(), is_registered)
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
        });
    }
}
