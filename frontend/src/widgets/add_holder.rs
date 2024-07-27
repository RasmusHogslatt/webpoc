use std::fmt::Debug;

use crate::{
    app_states::{AppState, HolderTypeSelection, ToolTypeSelection, WidgetState},
    singletons::Singletons,
};
use egui::{ComboBox, Context, Ui, Window};
use shared::{
    holders::holder::{
        BoringHeadSubcategory, ColletSubCategory, DrillChuckSubcategory, EndMillSubcategory,
        ExternalSubcategory, FormSubcategory, HydraulicSubcategory, InternalSubcategory,
        PartingGroovingSubcategory, QuickChangePostSubcategory, RotatingHolder,
        RotatingHolderCategory, ShellMillSubcategory, ShrinkFitSubcategory, TappingSubcategory,
        ThreadingSubcategory, TurningHolder, TurningHolderCategory,
    },
    magazine::Magazine,
    tools::tool::{
        Handedness, RotatingTool, RotatingToolCategory, TurningTool, TurningToolCategory,
    },
    User,
};

pub struct AddHolderWindow<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
    app_state: &'a mut AppState,
    widget_state: &'a mut WidgetState,
}

impl<'a> AddHolderWindow<'a> {
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

        Window::new("Add Holder")
            .default_width(800.0)
            .resizable(true)
            .open(open)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.singletons.holder_type_selection,
                        HolderTypeSelection::Rotating,
                        "Rotating",
                    );
                    ui.selectable_value(
                        &mut self.singletons.holder_type_selection,
                        HolderTypeSelection::Turning,
                        "Turning",
                    );
                });
                match self.singletons.holder_type_selection {
                    crate::app_states::HolderTypeSelection::Rotating => {
                        let rotating_holder = &mut self.singletons.rotating_holder;
                        // Choose category here
                        //
                        ComboBox::from_label("Category")
                            .selected_text(format!("{:?}", rotating_holder.category))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::Empty,
                                    "Empty",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::Collet(ColletSubCategory::Empty),
                                    "Ball Nose Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::EndMill(EndMillSubcategory::Empty),
                                    "Boring Tool",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::ShellMill(ShellMillSubcategory::Empty),
                                    "Chamfer Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::ShrinkFit(ShrinkFitSubcategory::Empty),
                                    "Dove Tail Cutter",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::Hydraulic(HydraulicSubcategory::Empty),
                                    "Drill Bit",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::DrillChuck(
                                        DrillChuckSubcategory::Empty,
                                    ),
                                    "End Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::BoringHead(
                                        BoringHeadSubcategory::Empty,
                                    ),
                                    "Face Mill",
                                );
                                ui.selectable_value(
                                    &mut rotating_holder.category,
                                    RotatingHolderCategory::Tapping(TappingSubcategory::Empty),
                                    "Reamer",
                                );
                            });

                        match &mut rotating_holder.category {
                            RotatingHolderCategory::Empty => {
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::Collet(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            ColletSubCategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ColletSubCategory::ER,
                                            "ER",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ColletSubCategory::TG,
                                            "TG",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ColletSubCategory::OZ,
                                            "OZ",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::EndMill(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            EndMillSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            EndMillSubcategory::WeldonFlat,
                                            "Weldon Flat",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            EndMillSubcategory::MillingChuck,
                                            "Milling Chuck",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::ShellMill(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            ShellMillSubcategory::Empty,
                                            "Empty",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::ShrinkFit(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            ShrinkFitSubcategory::Empty,
                                            "Empty",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::Hydraulic(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            HydraulicSubcategory::Empty,
                                            "Empty",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::DrillChuck(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            DrillChuckSubcategory::Empty,
                                            "Empty",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::BoringHead(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            BoringHeadSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            BoringHeadSubcategory::Adjustable,
                                            "Adjustable",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            BoringHeadSubcategory::MicroAdjustable,
                                            "Micro Adjustable",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                            RotatingHolderCategory::Tapping(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            TappingSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            TappingSubcategory::TensionCompression,
                                            "Tension Compression",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            TappingSubcategory::Rigid,
                                            "Rigid",
                                        );
                                    });
                                rotating_holder_settings_default(ui, rotating_holder);
                            }
                        }
                    }
                    crate::app_states::HolderTypeSelection::Turning => {
                        let turning_holder = &mut self.singletons.turning_holder;

                        ComboBox::from_label("Category")
                            .selected_text(format!("{:?}", turning_holder.category))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::Empty,
                                    "Empty",
                                );
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::Internal(InternalSubcategory::Empty),
                                    "Internal Turning Holder",
                                );
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::External(ExternalSubcategory::Empty),
                                    "External Turning Holder",
                                );
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::PartingGrooving(
                                        PartingGroovingSubcategory::Empty,
                                    ),
                                    "Parting/Grooving Holder",
                                );
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::Threading(ThreadingSubcategory::Empty),
                                    "Threading Holder",
                                );
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::Form(FormSubcategory::Empty),
                                    "Form Holder",
                                );
                                ui.selectable_value(
                                    &mut turning_holder.category,
                                    TurningHolderCategory::QuickChangePost(
                                        QuickChangePostSubcategory::Empty,
                                    ),
                                    "Quick-Change Post Holder",
                                );
                            });

                        match &mut turning_holder.category {
                            TurningHolderCategory::Empty => {
                                // TODO: Parameter ui here
                                turning_holder_settings_default(ui, turning_holder);
                            }
                            TurningHolderCategory::External(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            ExternalSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ExternalSubcategory::RightHand,
                                            "Right Hand",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ExternalSubcategory::LeftHand,
                                            "Left Hand",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ExternalSubcategory::Neutral,
                                            "Neutral",
                                        );
                                    });
                                turning_holder_settings_default(ui, turning_holder);
                            }
                            TurningHolderCategory::Internal(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            InternalSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            InternalSubcategory::BoringBar,
                                            "Boring Bar",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            InternalSubcategory::InternalThreading,
                                            "Internal Threading",
                                        );
                                    });
                                turning_holder_settings_default(ui, turning_holder);
                            }
                            TurningHolderCategory::PartingGrooving(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            PartingGroovingSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            PartingGroovingSubcategory::BladeType,
                                            "Blade Type",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            PartingGroovingSubcategory::CartridgeType,
                                            "Cartride Type",
                                        );
                                    });
                                turning_holder_settings_default(ui, turning_holder);
                            }
                            TurningHolderCategory::Threading(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            ThreadingSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ThreadingSubcategory::External,
                                            "External",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            ThreadingSubcategory::Internal,
                                            "Internal",
                                        );
                                    });
                                turning_holder_settings_default(ui, turning_holder);
                            }
                            TurningHolderCategory::Form(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            FormSubcategory::Empty,
                                            "Empty",
                                        );
                                    });
                                turning_holder_settings_default(ui, turning_holder);
                            }
                            TurningHolderCategory::QuickChangePost(subcategory) => {
                                ComboBox::from_label("Subcategory")
                                    .selected_text(format!("{:?}", subcategory))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            subcategory,
                                            QuickChangePostSubcategory::Empty,
                                            "Empty",
                                        );
                                        ui.selectable_value(
                                            subcategory,
                                            QuickChangePostSubcategory::QCTP,
                                            "QCTP",
                                        );
                                    });
                                turning_holder_settings_default(ui, turning_holder);
                            }
                        }
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("Add Holder").clicked() {
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

pub fn rotating_holder_settings_default(ui: &mut Ui, rotating_holder: &mut RotatingHolder) {
    egui::Grid::new("add_rotating_holder_default")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Diameter");
            ui.add(egui::DragValue::new(&mut rotating_holder.diameter));
            ui.end_row();
            ui.label("Length");
            ui.add(egui::DragValue::new(&mut rotating_holder.length));
            ui.end_row();
            ui.label("Maximum RPM");
            ui.add(egui::DragValue::new(&mut rotating_holder.max_rpm));
            ui.end_row();
            ui.label("Coolant Through");
            ui.checkbox(&mut rotating_holder.coolant_through, "Coolant through");
            ui.end_row();
            ui.label("Tool Clamping Range");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(
                    &mut rotating_holder.tool_clamping_range.0,
                ));
                ui.add(egui::DragValue::new(
                    &mut rotating_holder.tool_clamping_range.1,
                ));
            });
            ui.end_row();
            ui.label("Taper Type");
            ui.text_edit_singleline(&mut rotating_holder.taper_type);
            ui.end_row();
            ui.label("Runout");
            ui.add(egui::DragValue::new(&mut rotating_holder.runout));
            ui.end_row();
            ui.label("Balance Grade");
            ui.text_edit_singleline(&mut rotating_holder.balance_grade);
            ui.end_row();
            ui.label("Collet Type");
            ui.text_edit_singleline(&mut rotating_holder.collet_type);
            ui.end_row();
            ui.label("Weldon Flat Size");
            ui.add(egui::DragValue::new(&mut rotating_holder.weldon_flat_size));
            ui.end_row();
            ui.label("Adjustable Range");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(
                    &mut rotating_holder.adjustable_range.0,
                ));
                ui.add(egui::DragValue::new(
                    &mut rotating_holder.adjustable_range.1,
                ));
            });
            ui.end_row();
            ui.label("Tension Compression Range");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(
                    &mut rotating_holder.tension_compression_range.0,
                ));
                ui.add(egui::DragValue::new(
                    &mut rotating_holder.tension_compression_range.1,
                ));
            });
        });
}

pub fn turning_holder_settings_default(ui: &mut Ui, turning_holder: &mut TurningHolder) {
    egui::Grid::new("add_boring_tool")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Handedness");
            egui::ComboBox::from_label("")
                .selected_text(turning_holder.handedness.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut turning_holder.handedness,
                        Handedness::Neutral,
                        "Neutral",
                    );
                    ui.selectable_value(&mut turning_holder.handedness, Handedness::Left, "Left");
                    ui.selectable_value(&mut turning_holder.handedness, Handedness::Right, "Right");
                });
            ui.end_row();
            ui.label("Degree");
            ui.add(egui::DragValue::new(&mut turning_holder.degree));
            ui.end_row();
            ui.label("Shank Height");
            ui.add(egui::DragValue::new(&mut turning_holder.shank_height));
            ui.end_row();
            ui.label("Shank Width");
            ui.add(egui::DragValue::new(&mut turning_holder.shank_width));
            ui.end_row();
            ui.label("Overall Length");
            ui.add(egui::DragValue::new(&mut turning_holder.overall_length));
            ui.end_row();
            ui.label("Insert Size");
            ui.text_edit_singleline(&mut turning_holder.insert_size);
            ui.end_row();
            ui.label("Coolant Type");
            ui.text_edit_singleline(&mut turning_holder.coolant_type);
            ui.end_row();
            ui.label("Maximum Bore Depth");
            ui.add(egui::DragValue::new(&mut turning_holder.max_bore_depth));
            ui.end_row();
            ui.label("Minimum Bore Diameter");
            ui.add(egui::DragValue::new(&mut turning_holder.min_bore_diameter));
            ui.end_row();
            ui.label("Maximum Cutting Diameter");
            ui.add(egui::DragValue::new(
                &mut turning_holder.max_cutting_diameter,
            ));
            ui.end_row();
            ui.label("Quick-Change Compatible");
            ui.checkbox(
                &mut turning_holder.quick_change_compatible,
                "Quick-Change Compatible",
            );
            ui.end_row();
            ui.label("Cartridge Type");
            ui.text_edit_singleline(&mut turning_holder.cartridge_type);
            ui.end_row();
            ui.label("Thread Pitch Range");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(
                    &mut turning_holder.thread_pitch_range.0,
                ));
                ui.add(egui::DragValue::new(
                    &mut turning_holder.thread_pitch_range.1,
                ));
            });
            ui.end_row();
            ui.label("Form Profile");
            ui.text_edit_singleline(&mut turning_holder.form_profile);
            ui.end_row();
            ui.label("Tool Post Size");
            ui.text_edit_singleline(&mut turning_holder.tool_post_size);
        });
}
