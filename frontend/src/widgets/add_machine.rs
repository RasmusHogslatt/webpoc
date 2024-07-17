use crate::{
    app_states::{AppState, WidgetState},
    singletons::Singletons,
};
use egui::{Context, Ui, Window};
use shared::{magazine::Magazine, User};

pub struct AddMachineWindow<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
    app_state: &'a mut AppState,
    widget_state: &'a mut WidgetState,
}

impl<'a> AddMachineWindow<'a> {
    pub fn new(
        user: &'a mut User,
        singletons: &'a mut Singletons,
        app_state: &'a mut AppState,
        widget_state: &'a mut WidgetState,
    ) -> Self {
        Self {
            user,
            singletons,
            app_state,
            widget_state,
        }
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let mut should_close = false;

        Window::new("Add Machine").open(open).show(ctx, |ui| {
            ui.heading("Add Machine");
            ui.horizontal(|ui| {
                ui.label("Name: ");
                ui.text_edit_singleline(&mut self.singletons.machine.name);
            });
            ui.horizontal(|ui| {
                ui.label("Manufacturer: ");
                ui.text_edit_singleline(&mut self.singletons.machine.manufacturer);
            });
            ui.horizontal(|ui| {
                ui.label("Model: ");
                ui.text_edit_singleline(&mut self.singletons.machine.model);
            });
            ui.horizontal(|ui| {
                ui.label("Description: ");
                ui.text_edit_singleline(&mut self.singletons.description.text);
            });
            ui.horizontal(|ui| {
                ui.label("Number of Magazines");
                ui.add(
                    egui::widgets::Slider::new(&mut self.singletons.machine.magazine_count, 0..=10)
                        .text("Magazines"),
                );
            });
            if self.singletons.machine.magazine_count > 0 {
                ui.label("Magazine Capacity: ");
                ui.add(
                    egui::widgets::Slider::new(&mut self.singletons.magazine.capacity, 1..=100)
                        .text("Capacity"),
                );
            }
            ui.horizontal(|ui| {
                if ui.button("Add Machine").clicked() {
                    // Create magazines
                    for index in 0..self.singletons.machine.magazine_count {
                        self.singletons
                            .machine
                            .magazines
                            .push(Magazine::new(index, self.singletons.magazine.capacity));
                    }
                    self.user
                        .user_data
                        .machines
                        .push(self.singletons.machine.clone());
                    let index = self.user.user_data.machines.len() - 1;
                    self.user.user_data.selections.selected_machine = Some(index);
                    *self.app_state = AppState::Application;
                    *self.widget_state = WidgetState::Default;
                    should_close = true;
                    self.singletons.should_save_user_data = true;
                    // Reset singletons
                    self.singletons.Reset()
                }
                if ui.button("Cancel").clicked() {
                    *self.app_state = AppState::Application;
                    should_close = true;
                }
            });
        });

        if should_close {
            *open = false;
        }
    }
}
