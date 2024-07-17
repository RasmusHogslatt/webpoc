use crate::app_states::{AppState, OpenWindows, WidgetState};
#[cfg(target_arch = "wasm32")]
use crate::database_interactions::*;
use crate::settings::settings_sing_up::*;
use crate::singletons::Singletons;
use crate::widgets::add_machine::AddMachineWindow;
use crate::widgets::delete_machine::DeleteMachineWindow;
use crate::widgets::sign_in::SignInWidget;
use crate::widgets::sign_up::{show_status, SignUpWidget};
use crate::widgets::welcome::WelcomeWidget;
use eframe::App;
use egui::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use shared::custom_traits::*;
use shared::*;
use std::future::Future;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct Application {
    pub user: User,
    #[serde(skip)]
    pub client: Client,
    pub login_status: bool,
    pub registration_status: bool,
    pub app_state: AppState,
    pub widget_state: WidgetState,
    pub open_windows: OpenWindows,
    pub settings_sign_up: SettingsSignUp,
    pub singletons: Singletons,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            user: User::default(),
            login_status: false,
            registration_status: false,
            client: Client::new(),
            app_state: AppState::WelcomePage,
            widget_state: WidgetState::Default,
            open_windows: OpenWindows::default(),
            settings_sign_up: SettingsSignUp::default(),
            singletons: Singletons::default(),
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
}

impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.singletons.should_save_user_data {
            self.save_to_database(ctx);
            self.singletons.should_save_user_data = false;
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("App State: {:?}", self.app_state));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // User sign in/up
                    self.auth_combobox(ui);
                    if self.app_state == AppState::Application {
                        // Delete machine
                        if ui.button("Delete Current Machine").clicked()
                            && self.user.user_data.selections.selected_machine.is_some()
                        {
                            self.widget_state = WidgetState::DeleteMachine;
                            self.open_windows.delete_machine_window_open = true;
                        }
                        if let Some(machine_index) = self.user.user_data.selections.selected_machine
                        {
                            let mut delete_machine_window = DeleteMachineWindow::new(
                                &mut self.user,
                                &mut self.singletons,
                                &mut self.app_state,
                                &mut self.widget_state,
                                machine_index,
                            );
                            delete_machine_window
                                .show(ctx, &mut self.open_windows.delete_machine_window_open);
                        }
                        // Add machine
                        if ui.button("Add Machine").clicked() {
                            self.widget_state = WidgetState::AddMachine;
                            self.open_windows.add_machine_window_open = true;
                        }
                        let mut add_machine_window = AddMachineWindow::new(
                            &mut self.user,
                            &mut self.singletons,
                            &mut self.app_state,
                            &mut self.widget_state,
                        );
                        add_machine_window
                            .show(ctx, &mut self.open_windows.add_machine_window_open);
                        // Select machine
                        self.machines_combobox(ui);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.app_state {
                AppState::WelcomePage => {
                    WelcomeWidget::new(&mut self.app_state).ui(ui);
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
                    if login_status == Some(true) {
                        self.app_state = AppState::Application;
                        // Load the user data from memory
                        if let Some(user_data) =
                            ctx.memory(|mem| mem.data.get_temp::<UserData>("user_data".into()))
                        {
                            self.user.user_data = user_data;
                        }
                        // Remove temporary login status, but keep user data
                        ctx.memory_mut(|mem| {
                            mem.data.remove::<bool>("login_status".into());
                        });
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
                        self.user = User::default();
                        ctx.memory_mut(|mem| mem.data.remove::<bool>("registration_status".into()));
                    }
                }
                AppState::Application => {
                    if ui
                        .color_edit_button_srgba(&mut self.user.user_data.favorite_color)
                        .changed()
                    {
                        self.singletons.should_save_user_data = true;
                    }
                }
            };
        });
    }
}

impl Application {
    pub fn sign_out(&mut self) {
        self.singletons.should_save_user_data = true;
        self.app_state = AppState::WelcomePage;
        self.login_status = false;
        self.registration_status = false;
        self.user = User::default();
    }

    pub fn auth_combobox(&mut self, ui: &mut Ui) {
        match self.app_state {
            AppState::WelcomePage => {
                ComboBox::from_label("User actions")
                    .selected_text("First Use")
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Sign In").clicked() {
                            self.app_state = AppState::SignIn;
                        }
                        if ui.selectable_label(false, "Sign Up").clicked() {
                            self.app_state = AppState::SignUp;
                        }
                    });
            }
            AppState::SignIn => {
                ComboBox::from_label("Sign In")
                    .selected_text("Sign In")
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Sign Up").clicked() {
                            self.app_state = AppState::SignUp;
                        }
                        if ui.selectable_label(false, "Welcome Page").clicked() {
                            self.app_state = AppState::WelcomePage;
                        }
                    });
            }
            AppState::SignUp => {
                ComboBox::from_label("Sign Up")
                    .selected_text("Sign Up")
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Sign In").clicked() {
                            self.app_state = AppState::SignIn;
                        }
                        if ui.selectable_label(false, "Welcome Page").clicked() {
                            self.app_state = AppState::WelcomePage;
                        }
                    });
            }
            AppState::Application => {
                ComboBox::from_label("Signed in as:")
                    .selected_text(&self.user.username)
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Sign Out").clicked() {
                            self.singletons.should_save_user_data = true;
                            self.sign_out();
                        }
                    });
            }
        }
    }

    pub fn machines_combobox(&mut self, ui: &mut Ui) {
        let mut label_text = "".to_string();
        if self.user.user_data.machines.is_empty() {
            label_text = "No Machines Created".to_string();
        } else if self.user.user_data.selections.selected_machine.is_none() {
            label_text = "Select a Machine".to_string();
        }
        if self.user.user_data.selections.selected_machine.is_some()
            && !self.user.user_data.machines.is_empty()
        {
            let machine_reference = self
                .user
                .user_data
                .machines
                .get(self.user.user_data.selections.selected_machine.unwrap());
            label_text = machine_reference.unwrap().get_name();
        }

        ComboBox::from_label("Machine:")
            .selected_text(label_text.clone())
            .show_ui(ui, |ui| {
                for (i, machine) in self.user.user_data.machines.iter().enumerate() {
                    if ui
                        .selectable_label(
                            self.user.user_data.selections.selected_machine == Some(i),
                            machine.get_name(),
                        )
                        .clicked()
                    {
                        self.user.user_data.selections.selected_machine = Some(i);
                    }
                }
            });
    }
}
