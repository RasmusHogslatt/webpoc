use crate::{
    app_states::{AppState, WidgetState},
    singletons::Singletons,
};
use egui::{Context, Window};
use shared::User;

pub struct DeleteMachineWidget<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
    app_state: &'a mut AppState,
    widget_state: &'a mut WidgetState,
    machine_index: usize,
}

impl<'a> DeleteMachineWidget<'a> {
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

        Window::new("Delete Machine").open(open).show(ctx, |ui| {
            ui.heading("Delete Machine");

            if let Some(machine) = self.user.user_data.machines.get(self.machine_index) {
                ui.label(format!(
                    "Are you sure you want to delete the machine '{}'?",
                    machine.name
                ));

                ui.horizontal(|ui| {
                    if ui.button("Delete").clicked() {
                        self.user.user_data.machines.remove(self.machine_index);
                        // If the deleted machine was selected, clear the selection
                        if let Some(selected) = self.user.user_data.selections.selected_machine {
                            if selected == self.machine_index {
                                self.user.user_data.selections.selected_machine = None;
                            } else if selected > self.machine_index {
                                // Adjust the selection index if it was after the deleted machine
                                self.user.user_data.selections.selected_machine =
                                    Some(selected - 1);
                            }
                        }
                        *self.app_state = AppState::Application;
                        *self.widget_state = WidgetState::Default;
                        should_close = true;
                        self.singletons.should_save_user_data = true;
                    }
                    if ui.button("Cancel").clicked() {
                        *self.app_state = AppState::Application;
                        should_close = true;
                    }
                });
            } else {
                ui.label("Machine not found.");
                if ui.button("Close").clicked() {
                    should_close = true;
                }
            }
        });

        if should_close {
            *open = false;
        }
    }
}
