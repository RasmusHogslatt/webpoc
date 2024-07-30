use egui::{Context, Window};
use shared::User;

use crate::{
    app_states::{AppState, WidgetState},
    singletons::Singletons,
};

pub struct EditMachineWindow<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
    app_state: &'a mut AppState,
    widget_state: &'a mut WidgetState,
    machine_index: usize,
}

impl<'a> EditMachineWindow<'a> {
    pub fn new(
        user: &'a mut User,
        singletons: &'a mut Singletons,
        app_state: &'a mut AppState,
        widget_state: &'a mut WidgetState,
        machine_index: usize,
    ) -> Self {
        Self {
            user,
            singletons,
            app_state,
            widget_state,
            machine_index,
        }
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let mut should_close = false;

        Window::new("Edit Machine").open(open).show(ctx, |ui| {
            ui.heading("Edit Machine");
            let mut changed = false;
            let machine_index = self.user.user_data.selections.selected_machine.unwrap();
            if let Some(machine) = self.user.user_data.machines.get_mut(machine_index) {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    changed = ui.text_edit_singleline(&mut machine.name).changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Manufacturer:");
                    changed = ui.text_edit_singleline(&mut machine.manufacturer).changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Model:");
                    changed = ui.text_edit_singleline(&mut machine.model).changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Description:");
                    changed = ui
                        .text_edit_multiline(&mut machine.description.text)
                        .changed();
                });
                ui.label(format!("Number of magazines: {}", machine.magazines.len()));
                if machine.magazines.len() > 0 {
                    ui.label(format!(
                        "Magazine capacity: {}",
                        machine.magazines[0].capacity
                    ));
                }
                if changed {
                    self.singletons.should_save_user_data = true;
                }
            }
            if ui.button("Apply").clicked() {
                *self.app_state = AppState::Application;
                *self.widget_state = WidgetState::Default;
                should_close = true;
            }
        });
        if should_close {
            *open = false;
        }
    }
}
