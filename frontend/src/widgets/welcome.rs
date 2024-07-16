use crate::app_states::AppState;
use egui::{Response, Ui, Widget};

pub struct WelcomeWidget<'a> {
    app_state: &'a mut AppState,
}

impl<'a> WelcomeWidget<'a> {
    pub fn new(app_state: &'a mut AppState) -> Self {
        Self { app_state }
    }
}

impl Widget for WelcomeWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical_centered(|ui| {
            ui.group(|ui| {
                ui.heading("Welcome to Dad's Machine Application!");
                ui.add_space(10.0);
                ui.label(
                    "The place where your dreams come true and your machines are always running!",
                );

                ui.add_space(5.0); // Optional: adds a little space above the buttons
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // ui.label("Please sign in or sign up to continue:");
                    if ui.button("Sign In").clicked() {
                        *self.app_state = AppState::SignIn;
                    }
                    if ui.button("Sign Up").clicked() {
                        *self.app_state = AppState::SignUp;
                    }
                });
            });
        })
        .response
    }
}
