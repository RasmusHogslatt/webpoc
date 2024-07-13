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
        })
        .response
    }
}
