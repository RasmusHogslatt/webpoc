use crate::app_states::AppState;
use egui::{Response, Ui, Widget};

pub struct FirstUseWidget<'a> {
    app_state: &'a mut AppState,
}

impl<'a> FirstUseWidget<'a> {
    pub fn new(app_state: &'a mut AppState) -> Self {
        Self { app_state }
    }
}

impl Widget for FirstUseWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.heading("Welcome to the app!");
            ui.add_space(10.0);

            let signup_response = ui.button("Sign Up");
            if signup_response.clicked() {
                *self.app_state = AppState::SignUp;
            }

            ui.add_space(5.0);

            let signin_response = ui.button("Sign In");
            if signin_response.clicked() {
                *self.app_state = AppState::SignIn;
            }

            signup_response.union(signin_response)
        })
        .response
    }
}
