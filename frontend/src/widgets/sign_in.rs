use crate::app_states::AppState;
use egui::{Response, Ui, Widget};
use shared::User;

pub struct SignInWidget<'a> {
    user: &'a mut User,
    on_submit: &'a dyn Fn(&User),
    app_state: &'a mut AppState,
}

impl<'a> SignInWidget<'a> {
    pub fn new(
        user: &'a mut User,
        app_state: &'a mut AppState,
        on_submit: &'a dyn Fn(&User),
    ) -> Self {
        Self {
            user,
            app_state,
            on_submit,
        }
    }
}

impl<'a> Widget for SignInWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.heading("Sign In");
            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.user.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.text_edit_singleline(&mut self.user.password);
            });
            ui.horizontal(|ui| {
                if ui.button("Sign In").clicked() {
                    (self.on_submit)(self.user);
                }
                if ui.button("Cancel").clicked() {
                    *self.app_state = AppState::WelcomePage;
                }
            });
        })
        .response
    }
}
