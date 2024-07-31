use crate::{
    app_states::{AppState, WidgetState},
    singletons::Singletons,
};
use egui::{Context, Window};
use shared::{magazine::Magazine, settings::Settings, User};

pub struct SettingsWindow<'a> {
    settings: &'a mut Settings,
    singletons: &'a mut Singletons,
    widget_state: &'a mut WidgetState,
}

impl<'a> SettingsWindow<'a> {
    pub fn new(
        settings: &'a mut Settings,
        singletons: &'a mut Singletons,
        widget_state: &'a mut WidgetState,
    ) -> Self {
        Self {
            settings,
            singletons,
            widget_state,
        }
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let mut should_close = false;

        Window::new("Settings").open(open).show(ctx, |ui| {
            ui.heading("This is where I change my settings");

            if ui.button("Cancel").clicked() {
                *self.widget_state = WidgetState::Default;
                should_close = true;
            }
        });

        if should_close {
            *open = false;
        }
    }
}
