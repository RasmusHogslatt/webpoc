use crate::app_states::AppState;
use crate::settings::settings_sing_up::*;
use crate::widgets::first_use::FirstUseWidget;
use crate::widgets::sign_in::SignInWidget;
use crate::widgets::sign_up::{show_status, SignUpWidget};
use egui::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::*;
use std::future::Future;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    user: User,
    #[serde(skip)]
    client: Client,
    login_status: Option<bool>,
    registration_status: Option<bool>,
    app_state: AppState,
    settings_sign_up: SettingsSignUp,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            user: User::default(),
            login_status: None,
            registration_status: None,
            client: Client::new(),
            app_state: AppState::FirstUse,
            settings_sign_up: SettingsSignUp::default(),
        }
    }
}

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

impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

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
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("App State: {:?}", self.app_state));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.app_state {
                AppState::FirstUse => {
                    FirstUseWidget::new(&mut self.app_state).ui(ui);
                }
                AppState::SignIn => {
                    SignInWidget::new(&mut self.user, &mut self.app_state, &|user| {
                        let username = user.username.clone();
                        let password = user.password.clone();
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
                    })
                    .ui(ui);
                    let login_status = ctx.memory(|mem| mem.data.get_temp("login_status".into()));
                    show_status(ui, login_status, "Login successful!", "Login failed!");
                    if login_status.is_some() && login_status.unwrap() {
                        self.app_state = AppState::Application;
                    }
                }
                AppState::SignUp => {
                    SignUpWidget::new(
                        &mut self.user,
                        &mut self.settings_sign_up.show_password,
                        &mut self.app_state,
                        &|user| {
                            let username = user.username.clone();
                            let password = user.password.clone();
                            let email = user.email.clone();
                            let client = self.client.clone();
                            let ctx = ctx.clone();

                            spawn_task(async move {
                                match Self::register_user(username, password, email, client).await {
                                    Ok(is_registered) => {
                                        ctx.request_repaint();
                                        ctx.memory_mut(|mem| {
                                            mem.data.insert_temp(
                                                "registration_status".into(),
                                                is_registered,
                                            );
                                        });
                                    }
                                    Err(e) => {
                                        let error_message = format!("Registration failed: {:?}", e);
                                        println!("{}", error_message);
                                        ctx.request_repaint();
                                        ctx.memory_mut(|mem| {
                                            mem.data
                                                .insert_temp("registration_status".into(), false);
                                            mem.data
                                                .insert_temp("error_message".into(), error_message);
                                        });
                                    }
                                }
                            });
                        },
                    )
                    .ui(ui);
                    let registration_status =
                        ctx.memory(|mem| mem.data.get_temp("registration_status".into()));
                    show_status(
                        ui,
                        registration_status,
                        "Registration successful!",
                        "Registration failed!",
                    );
                    if registration_status.is_some() && registration_status.unwrap() {
                        self.app_state = AppState::SignIn;
                    }
                }
                AppState::Application => {
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

                    ui.colored_label(
                        self.user.user_data.favorite_color,
                        format!("{:?}", self.user.user_data.favorite_color),
                    );
                    if ui.button("Go home").clicked() {
                        self.app_state = AppState::FirstUse;
                    }
                    let update_status = ctx.memory(|mem| mem.data.get_temp("update_status".into()));
                    show_status(
                        ui,
                        update_status,
                        "User data updated successfully!",
                        "Failed to update user data!",
                    );
                }
            };
        });
    }
}
