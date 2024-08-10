use crate::{
    app_states::{AppState, WidgetState},
    singletons::Singletons,
};
use egui::{color_picker::Alpha, Context, Window};
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

        Window::new("Settings")
            .resizable(false)
            .open(open)
            .show(ctx, |ui| {
                egui::Grid::new("settings_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Color 1:");
                        ui.color_edit_button_srgba(&mut self.settings.color1);
                        ui.end_row();
                        ui.label("Color 2:");
                        ui.color_edit_button_srgba(&mut self.settings.color2);
                        ui.end_row();
                        ui.label("Color 3:");
                        ui.color_edit_button_srgba(&mut self.settings.color3);
                        ui.end_row();
                    });

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
