use egui::{Color32, Response, TextEdit, Ui, Widget};
use shared::User;

pub struct SignUpWidget<'a> {
    user: &'a mut User,
    on_submit: &'a dyn Fn(&User),
    pub show_password: bool,
}

impl<'a> SignUpWidget<'a> {
    pub fn new(user: &'a mut User, on_submit: &'a dyn Fn(&User)) -> Self {
        Self {
            user,
            on_submit,
            show_password: false,
        }
    }
}

impl<'a> Widget for SignUpWidget<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ui.heading("Sign Up");
            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.user.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.text_edit_singleline(&mut self.user.password);
            });
            ui.horizontal(|ui| {
                ui.label("Email: ");
                ui.add(TextEdit::singleline(&mut self.user.email).password(self.show_password));
                ui.checkbox(&mut self.show_password, "Show password");
            });
            if ui.button("Register").clicked() {
                (self.on_submit)(self.user);
            }
        })
        .response
    }
}

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
