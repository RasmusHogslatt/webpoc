use std::fmt::Debug;

use crate::{
    app_states::{AppState, ToolTypeSelection, WidgetState},
    singletons::Singletons,
};
use egui::{ComboBox, Context, Ui, Window};
use shared::{
    custom_traits::SetSlot,
    magazine::Magazine,
    tools::tool::{
        Handedness, RotatingTool, RotatingToolCategory, TurningTool, TurningToolCategory,
    },
    User,
};
use uuid::Uuid;

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

        Window::new("Add Tool")
            .default_width(800.0)
            .resizable(true)
            .open(open)
            .show(ctx, |ui| {
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
                        let rotating_tool = &mut self.singletons.rotating_tool;
                        // Choose category here
                        //
                        ComboBox::from_label("Category")
                            .selected_text(format!("{:?}", rotating_tool.category))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::Empty,
                                    "Empty",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::BallNoseMill,
                                    "Ball Nose Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::BoringTool,
                                    "Boring Tool",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::ChamferMill,
                                    "Chamfer Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::DoveTailCutter,
                                    "Dove Tail Cutter",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::DrillBit,
                                    "Drill Bit",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::EndMill,
                                    "End Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::FaceMill,
                                    "Face Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::Reamer,
                                    "Reamer",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::SlotDrill,
                                    "Slot Drill",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::ThreadMill,
                                    "Thread Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_tool.category,
                                    RotatingToolCategory::TSlotCutter,
                                    "T-Slot Cutter",
                                );
                            });

                        // Category should be chosen before here
                        match rotating_tool.category {
                            shared::tools::tool::RotatingToolCategory::Empty => {
                                ui.heading("Any category");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::BallNoseMill => {
                                ui.heading("BallNose");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::BoringTool => {
                                ui.heading("Boring");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::ChamferMill => {
                                ui.heading("Chamfer");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::DoveTailCutter => {
                                ui.heading("DoveTail");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::DrillBit => {
                                ui.heading("DrillBit");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::EndMill => {
                                ui.heading("EndMill");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::FaceMill => {
                                ui.heading("FaceMill");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::Reamer => {
                                ui.heading("Reamer");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::SlotDrill => {
                                ui.heading("SlotDrill");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::ThreadMill => {
                                ui.heading("ThreadMill");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                            shared::tools::tool::RotatingToolCategory::TSlotCutter => {
                                ui.heading("TSlotCutter");
                                rotating_tool_settings_default(ui, rotating_tool);
                            }
                        }
                        // Match category, empty shows all fields
                    }
                    crate::app_states::ToolTypeSelection::Turning => {
                        // TODO
                        let turning_tool = &mut self.singletons.turning_tool;

                        ComboBox::from_label("Category")
                            .selected_text(format!("{:?}", turning_tool.category))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::Empty,
                                    "Empty",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::InternalTurningTool,
                                    "Internal Turning Tool",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::ExternalTurningTool,
                                    "External Turning Tool",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::FacingTool,
                                    "Facing Tool",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::BoringBar,
                                    "Boring Bar",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::ThreadingTool,
                                    "Threading Tool",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::GroovingPartingTool,
                                    "Grooving/Parting Tool",
                                );
                                ui.selectable_value(
                                    &mut turning_tool.category,
                                    TurningToolCategory::FormTool,
                                    "Form Tool",
                                );
                            });

                        match turning_tool.category {
                            TurningToolCategory::Empty => {
                                ui.heading("Any Category");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::InternalTurningTool => {
                                ui.heading("Internal Turning Tool");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::ExternalTurningTool => {
                                ui.heading("External Turning Tool");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::FacingTool => {
                                ui.heading("Facing Tool");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::BoringBar => {
                                ui.heading("Boring Bar");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::ThreadingTool => {
                                ui.heading("Threading Tool");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::GroovingPartingTool => {
                                ui.heading("Grooving/Parting Tool");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                            TurningToolCategory::FormTool => {
                                ui.heading("Form Tool");
                                turning_tool_settings_default(ui, turning_tool)
                            }
                        }
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("Add Tool").clicked() {
                        match self.singletons.tool_type_selection {
                            ToolTypeSelection::Rotating => {
                                let mut tool = self.singletons.rotating_tool.clone();
                                let index = self.user.user_data.library.tools.len();
                                let uuid = Uuid::new_v4().to_string();
                                tool.uuid = uuid;
                                tool.set_library_slot(Some(index));
                                self.user
                                    .user_data
                                    .library
                                    .tools
                                    .push(shared::tools::tool::Tool::Rotating(tool));
                            }
                            ToolTypeSelection::Turning => {
                                let mut tool = self.singletons.turning_tool.clone();
                                let index = self.user.user_data.library.tools.len();
                                let uuid = Uuid::new_v4().to_string();
                                tool.uuid = uuid;
                                tool.set_library_slot(Some(index));
                                self.user
                                    .user_data
                                    .library
                                    .tools
                                    .push(shared::tools::tool::Tool::Turning(tool));
                            }
                        }
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

pub fn rotating_tool_settings_default(ui: &mut Ui, rotating_tool: &mut RotatingTool) {
    egui::Grid::new("add_rotating_tool_default")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Cutting diameter");
            ui.add(egui::DragValue::new(&mut rotating_tool.cutting_diameter));
            ui.end_row();
            ui.label("Connection diameter");
            ui.add(egui::DragValue::new(&mut rotating_tool.connection_diameter));
            ui.end_row();
            ui.label("Usable length");
            ui.add(egui::DragValue::new(&mut rotating_tool.usable_length));
            ui.end_row();
            ui.label("Functional length");
            ui.add(egui::DragValue::new(&mut rotating_tool.functional_length));
            ui.end_row();
            ui.label("Tool weight");
            ui.add(egui::DragValue::new(&mut rotating_tool.weight_of_tool));
            ui.end_row();
            ui.label("Max RPM");
            ui.add(egui::DragValue::new(&mut rotating_tool.max_rpm));
            ui.end_row();
            ui.label("Coolant pressure");
            ui.add(egui::DragValue::new(&mut rotating_tool.coolant_pressure));
            ui.end_row();
            ui.label("Achievable hole tolerance");
            ui.add(egui::DragValue::new(
                &mut rotating_tool.achievable_hole_tolerance,
            ));
        });
}

pub fn turning_tool_settings_default(ui: &mut Ui, turning_tool: &mut TurningTool) {
    egui::Grid::new("add_turning_tool_default")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Handedness");
            egui::ComboBox::from_label("")
                .selected_text(turning_tool.handedness.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut turning_tool.handedness,
                        Handedness::Neutral,
                        "Neutral",
                    );
                    ui.selectable_value(&mut turning_tool.handedness, Handedness::Left, "Left");
                    ui.selectable_value(&mut turning_tool.handedness, Handedness::Right, "Right");
                });
            ui.end_row();
            ui.label("Insert type");
            ui.text_edit_singleline(&mut turning_tool.insert_type);
            ui.end_row();
            ui.label("Cutting edge angle");
            ui.add(egui::DragValue::new(&mut turning_tool.cutting_edge_angle));
            ui.end_row();
            ui.label("Maximum ramping angle");
            ui.add(egui::DragValue::new(
                &mut turning_tool.maximum_ramping_angle,
            ));
            ui.end_row();
            ui.label("Minimum bode diameter");
            ui.add(egui::DragValue::new(&mut turning_tool.usable_length));
            ui.end_row();
            ui.label("Workpiece side body angle");
            ui.add(egui::DragValue::new(
                &mut turning_tool.workpiece_side_body_angle,
            ));
            ui.end_row();
            ui.label("Maximum cutting depth");
            ui.add(egui::DragValue::new(
                &mut turning_tool.cutting_depth_maximum,
            ));
            ui.end_row();
            ui.label("Machine side body angle");
            ui.add(egui::DragValue::new(
                &mut turning_tool.machine_side_body_angle,
            ));
            ui.end_row();
            ui.label("Minimum overhang");
            ui.add(egui::DragValue::new(&mut turning_tool.minimum_overhang));
            ui.end_row();
            ui.label("Maximum overhang");
            ui.add(egui::DragValue::new(&mut turning_tool.maximum_overhang));
            ui.end_row();
            ui.label("Usable length");
            ui.add(egui::DragValue::new(&mut turning_tool.usable_length));
            ui.end_row();
            ui.label("Body length");
            ui.add(egui::DragValue::new(&mut turning_tool.body_length));
            ui.end_row();
            ui.label("Body diameter");
            ui.add(egui::DragValue::new(&mut turning_tool.body_diameter));
            ui.end_row();
            ui.label("Functional diameter");
            ui.add(egui::DragValue::new(&mut turning_tool.functional_diameter));
            ui.end_row();
            ui.label("Peripheral effective cutting");
            ui.add(egui::DragValue::new(
                &mut turning_tool.peripheral_effective_cutting,
            ));
            ui.end_row();
            ui.label("Connection diameter");
            ui.add(egui::DragValue::new(&mut turning_tool.connection_diameter));
            ui.end_row();
            ui.label("Maximum RPM");
            ui.add(egui::DragValue::new(&mut turning_tool.maximum_rpm));
            ui.end_row();
            ui.label("Tool weight");
            ui.add(egui::DragValue::new(&mut turning_tool.tool_weight));
        });
}
