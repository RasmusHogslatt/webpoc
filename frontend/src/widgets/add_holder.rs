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
        PartingGroovingSubcategory, QuickChangePostSubcategory, RotatingHolderCategory,
        ShellMillSubcategory, ShrinkFitSubcategory, TappingSubcategory, ThreadingSubcategory,
        TurningHolderCategory,
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
                                // TODO: Add parameters here
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
                                // TODO: Ui for parameters here
                            }
                        }
                    }
                    crate::app_states::HolderTypeSelection::Turning => {
                        // TODO
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
                            }
                        }
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
