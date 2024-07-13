use crate::app_states::AppState;
use egui::{Color32, Response, TextEdit, Ui, Widget};
use shared::User;

pub struct SignUpWidget<'a> {
    user: &'a mut User,
    show_password: &'a mut bool,
    on_submit: &'a dyn Fn(&User),
    app_state: &'a mut AppState,
}

impl<'a> SignUpWidget<'a> {
    pub fn new(
        user: &'a mut User,
        show_password: &'a mut bool,
        app_state: &'a mut AppState,
        on_submit: &'a dyn Fn(&User),
    ) -> Self {
        Self {
            user,
            show_password,
            app_state,
            on_submit,
        }
    }
}

impl<'a> Widget for SignUpWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.heading("Sign Up");
            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.user.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.add(
                    TextEdit::singleline(&mut self.user.password).password(!*self.show_password),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Email: ");
                ui.text_edit_singleline(&mut self.user.email);
            });
            ui.checkbox(self.show_password, "Show password");
            if ui.button("Register").clicked() {
                (self.on_submit)(self.user);
            }
        })
        .response
    }
}

// The show_status function remains unchanged
pub fn show_status(
    ui: &mut Ui,
    status: Option<bool>,
    success_message: &str,
    failure_message: &str,
) {
    if let Some(status) = status {
        ui.colored_label(
            if status { Color32::GREEN } else { Color32::RED },
            if status {
                success_message
            } else {
                failure_message
            },
        );
    }
}
