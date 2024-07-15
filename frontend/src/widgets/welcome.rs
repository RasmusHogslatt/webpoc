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
        // welcome heading in center of the screen
        ui.centered_and_justified(|ui| {
            ui.heading("Welcome to Dad's Machine Application!");
            ui.end_row();
            ui.heading(
                "The place where your dreams come true and your machines are always running!",
            );
            // button to sign in to the left and sign up to the right
            ui.horizontal(|ui| {
                if ui.button("Sign In").clicked() {
                    *self.app_state = AppState::SignIn;
                }
                if ui.button("Sign Up").clicked() {
                    *self.app_state = AppState::SignUp;
                }
            });
        })
        .response
    }
}
