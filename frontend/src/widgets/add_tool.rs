use crate::{
    app_states::{AppState, ToolTypeSelection, WidgetState},
    singletons::Singletons,
};
use egui::{Context, Window};
use shared::{magazine::Magazine, User};

pub struct AddToolWindow<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
    app_state: &'a mut AppState,
    widget_state: &'a mut WidgetState,
}

impl<'a> AddToolWindow<'a> {
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

        Window::new("Add Tool").open(open).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.singletons.tool_type_selection,
                    ToolTypeSelection::Rotating,
                    "Rotating",
                );
                ui.selectable_value(
                    &mut self.singletons.tool_type_selection,
                    ToolTypeSelection::Turning,
                    "Turning",
                );
            });
            match self.singletons.tool_type_selection {
                crate::app_states::ToolTypeSelection::Rotating => {
                    ui.label("Add Rotating");
                    let rotating_tool = &mut self.singletons.rotating_tool;
                    // Choose category here
                    //
                    // Category should be chosen before here
                    match rotating_tool.category {
                        shared::tools::tool::RotatingToolCategory::Empty => {
                            ui.add(egui::DragValue::new(&mut rotating_tool.diameter));
                        }
                        shared::tools::tool::RotatingToolCategory::BallNoseMill => todo!(),
                        shared::tools::tool::RotatingToolCategory::BoringTool => todo!(),
                        shared::tools::tool::RotatingToolCategory::ChamferMill => todo!(),
                        shared::tools::tool::RotatingToolCategory::DoveTailCutter => todo!(),
                        shared::tools::tool::RotatingToolCategory::DrillBit => todo!(),
                        shared::tools::tool::RotatingToolCategory::EndMill => todo!(),
                        shared::tools::tool::RotatingToolCategory::FaceMill => todo!(),
                        shared::tools::tool::RotatingToolCategory::Reamer => todo!(),
                        shared::tools::tool::RotatingToolCategory::SlotDrill => todo!(),
                        shared::tools::tool::RotatingToolCategory::ThreadMill => todo!(),
                        shared::tools::tool::RotatingToolCategory::TSlotCutter => todo!(),
                    }
                    // Match category, empty shows all fields
                }
                crate::app_states::ToolTypeSelection::Turning => {
                    ui.label("Add Turning");
                }
            }

            ui.horizontal(|ui| {
                if ui.button("Add Tool").clicked() {
                    *self.app_state = AppState::Application;
                    *self.widget_state = WidgetState::Default;
                    should_close = true;
                    self.singletons.should_save_user_data = true;
                    // Reset singletons
                    self.singletons.reset()
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
