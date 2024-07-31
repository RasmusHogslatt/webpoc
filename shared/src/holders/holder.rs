use std::fmt;

use egui::Ui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    custom_traits::{
        AddHolderCopy, DeleteHolder, GetHolderType, GetRotatingHolderCategory,
        GetTurningHolderCategory, GetUuid, UiDisplay,
    },
    tools::tool::Handedness,
};

// Highest level holder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Holder {
    Rotating(RotatingHolder),
    Turning(TurningHolder),
}

impl AddHolderCopy for Holder {
    fn add_copy(&mut self) {
        match self {
            Holder::Rotating(x) => x.add_copy(),
            Holder::Turning(x) => x.add_copy(),
        }
    }
}

impl AddHolderCopy for RotatingHolder {
    fn add_copy(&mut self) {
        self.duplicates = self.duplicates + 1;
    }
}

impl AddHolderCopy for TurningHolder {
    fn add_copy(&mut self) {
        self.duplicates = self.duplicates + 1;
    }
}

impl DeleteHolder for Holder {
    fn delete_holder(&mut self) -> bool {
        match self {
            Holder::Rotating(x) => x.delete_holder(),
            Holder::Turning(x) => x.delete_holder(),
        }
    }
}

impl DeleteHolder for RotatingHolder {
    fn delete_holder(&mut self) -> bool {
        if self.duplicates > 1 {
            self.duplicates = self.duplicates - 1;
            false
        } else {
            true
        }
    }
}

impl DeleteHolder for TurningHolder {
    fn delete_holder(&mut self) -> bool {
        if self.duplicates > 1 {
            self.duplicates = self.duplicates - 1;
            false
        } else {
            true
        }
    }
}

impl GetUuid for Holder {
    fn get_uuid(&self) -> String {
        match self {
            Holder::Rotating(x) => x.get_uuid(),
            Holder::Turning(x) => x.get_uuid(),
        }
    }
}

impl GetUuid for RotatingHolder {
    fn get_uuid(&self) -> String {
        self.uuid.to_string()
    }
}

impl GetUuid for TurningHolder {
    fn get_uuid(&self) -> String {
        self.uuid.to_string()
    }
}

impl UiDisplay for Holder {
    fn display(&self, ui: &mut egui::Ui) {
        match self {
            Holder::Rotating(item) => item.display(ui),
            Holder::Turning(item) => item.display(ui),
        }
    }
}

fn rotating_hover_ui(ui: &mut Ui, rotating_holder: &RotatingHolder) {
    egui::Grid::new("rotating_holder_info")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Adjustable Range:");
            ui.label(format!("{:?}", rotating_holder.adjustable_range));
            ui.end_row();
            ui.label("Balance Grade:");
            ui.label(&rotating_holder.balance_grade);
            ui.end_row();
            ui.label("Collet Type:");
            ui.label(&rotating_holder.collet_type);
            ui.end_row();
            ui.label("Coolant Through:");
            ui.label(format!("{}", rotating_holder.coolant_through));
            ui.end_row();
            ui.label("Diameter:");
            ui.label(format!("{:.2} mm", rotating_holder.diameter));
            ui.end_row();
            ui.label("Length:");
            ui.label(format!("{:.2} mm", rotating_holder.length));
            ui.end_row();
            ui.label("Maximum RPM:");
            ui.label(format!("{}", rotating_holder.max_rpm));
            ui.end_row();
            ui.label("Runout:");
            ui.label(format!("{:.3} mm", rotating_holder.runout));
            ui.end_row();
            ui.label("Taper Type:");
            ui.label(&rotating_holder.taper_type);
            ui.end_row();
            ui.label("Tension Compression Range:");
            ui.label(format!("{:?}", rotating_holder.tension_compression_range));
            ui.end_row();
            ui.label("Tool Clamping Range:");
            ui.label(format!("{:?} mm", rotating_holder.tool_clamping_range));
            ui.end_row();
            ui.label("Weldon Flat Size:");
            ui.label(format!("{:.2} mm", rotating_holder.weldon_flat_size));
            ui.end_row();
        });
}

impl UiDisplay for RotatingHolder {
    fn display(&self, ui: &mut egui::Ui) {
        let response = ui.group(|ui| {
            // Create a unique ID for each grid based on the RotatingHolder's properties
            let grid_id = egui::Id::new(format!("rotating_holder_grid_{}", self.uuid));

            egui::Grid::new(grid_id)
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Category:");
                    ui.label(format!("{}", self.category));
                    ui.end_row();

                    ui.label("Copies:");
                    ui.label(format!("{}", self.duplicates));
                    ui.end_row();

                    ui.label("Diameter:");
                    ui.label(format!("{:.2} mm", self.diameter));
                    ui.end_row();

                    ui.label("Length:");
                    ui.label(format!("{:.2} mm", self.length));
                    ui.end_row();

                    match &self.category {
                        RotatingHolderCategory::Empty => {}
                        RotatingHolderCategory::Collet(subcategory) => {
                            ui.label("Collet Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::EndMill(subcategory) => {
                            ui.label("End Mill Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::ShellMill(subcategory) => {
                            ui.label("Shell Mill Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::ShrinkFit(subcategory) => {
                            ui.label("Shrink Fit Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::Hydraulic(subcategory) => {
                            ui.label("Hydraulic Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::DrillChuck(subcategory) => {
                            ui.label("Drill Chuck Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::BoringHead(subcategory) => {
                            ui.label("Boring Head Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        RotatingHolderCategory::Tapping(subcategory) => {
                            ui.label("Tapping Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                    }
                });
        });

        // Add hover effect for all cases
        response.response.on_hover_ui(|ui| {
            rotating_hover_ui(ui, self);
        });
    }
}

fn turning_hover_ui(ui: &mut Ui, turning_holder: &TurningHolder) {
    egui::Grid::new("turning_holder_info")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Degree:");
            ui.label(format!("{:.2}°", turning_holder.degree));
            ui.end_row();
            ui.label("Shank Height:");
            ui.label(format!("{:.2} mm", turning_holder.shank_height));
            ui.end_row();
            ui.label("Shank Width:");
            ui.label(format!("{:.2} mm", turning_holder.shank_width));
            ui.end_row();
            ui.label("Overall Length:");
            ui.label(format!("{:.2} mm", turning_holder.overall_length));
            ui.end_row();
            ui.label("Insert Size:");
            ui.label(&turning_holder.insert_size);
            ui.end_row();
            ui.label("Handedness:");
            ui.label(format!("{:?}", turning_holder.handedness));
            ui.end_row();
            ui.label("Coolant Type:");
            ui.label(&turning_holder.coolant_type);
            ui.end_row();
            ui.label("Max Bore Depth:");
            ui.label(format!("{:.2} mm", turning_holder.max_bore_depth));
            ui.end_row();
            ui.label("Min Bore Diameter:");
            ui.label(format!("{:.2} mm", turning_holder.min_bore_diameter));
            ui.end_row();
            ui.label("Max Cutting Diameter:");
            ui.label(format!("{:.2} mm", turning_holder.max_cutting_diameter));
            ui.end_row();
            ui.label("Quick Change Compatible:");
            ui.label(format!("{}", turning_holder.quick_change_compatible));
            ui.end_row();
            ui.label("Cartridge Type:");
            ui.label(&turning_holder.cartridge_type);
            ui.end_row();
            ui.label("Thread Pitch Range:");
            ui.label(format!("{:?} mm", turning_holder.thread_pitch_range));
            ui.end_row();
            ui.label("Form Profile:");
            ui.label(&turning_holder.form_profile);
            ui.end_row();
            ui.label("Tool Post Size:");
            ui.label(&turning_holder.tool_post_size);
            ui.end_row();
        });
}

impl UiDisplay for TurningHolder {
    fn display(&self, ui: &mut egui::Ui) {
        let response = ui.group(|ui| {
            // Create a unique ID for each grid based on the TurningHolder's properties
            let grid_id = egui::Id::new(format!("turning_holder_grid_{}", self.uuid));

            egui::Grid::new(grid_id)
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Category:");
                    ui.label(format!("{}", self.category));
                    ui.end_row();
                    ui.label("Copies:");
                    ui.label(format!("{}", self.duplicates));
                    ui.end_row();

                    ui.label("Shank:");
                    ui.label(format!(
                        "{:.2}x{:.2} mm",
                        self.shank_width, self.shank_height
                    ));
                    ui.end_row();

                    ui.label("Overall Length:");
                    ui.label(format!("{:.2} mm", self.overall_length));
                    ui.end_row();

                    match &self.category {
                        TurningHolderCategory::Empty => {}
                        TurningHolderCategory::External(subcategory) => {
                            ui.label("External Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        TurningHolderCategory::Internal(subcategory) => {
                            ui.label("Internal Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        TurningHolderCategory::PartingGrooving(subcategory) => {
                            ui.label("Parting/Grooving Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        TurningHolderCategory::Threading(subcategory) => {
                            ui.label("Threading Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        TurningHolderCategory::Form(subcategory) => {
                            ui.label("Form Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                        TurningHolderCategory::QuickChangePost(subcategory) => {
                            ui.label("Quick Change Post Type:");
                            ui.label(format!("{}", subcategory));
                            ui.end_row();
                        }
                    }
                });
        });

        // Add hover effect for all cases
        response.response.on_hover_ui(|ui| {
            turning_hover_ui(ui, self);
        });
    }
}

impl GetHolderType for Holder {
    fn is_rotating(&self) -> bool {
        match self {
            Holder::Rotating(_) => true,
            Holder::Turning(_) => false,
        }
    }
    fn is_turning(&self) -> bool {
        match self {
            Holder::Rotating(_) => false,
            Holder::Turning(_) => true,
        }
    }
}

impl Default for Holder {
    fn default() -> Self {
        Holder::Rotating(RotatingHolder::default())
    }
}

impl GetRotatingHolderCategory for Holder {
    fn get_rotating_holder_category(&self) -> Option<RotatingHolderCategory> {
        match self {
            Holder::Rotating(item) => item.get_rotating_holder_category(),
            Holder::Turning(_) => None,
        }
    }
}

impl GetRotatingHolderCategory for RotatingHolder {
    fn get_rotating_holder_category(&self) -> Option<RotatingHolderCategory> {
        Some(self.category.clone())
    }
}

impl GetTurningHolderCategory for Holder {
    fn get_turning_holder_category(&self) -> Option<TurningHolderCategory> {
        match self {
            Holder::Rotating(_) => None,
            Holder::Turning(item) => item.get_turning_holder_category(),
        }
    }
}

impl GetTurningHolderCategory for TurningHolder {
    fn get_turning_holder_category(&self) -> Option<TurningHolderCategory> {
        Some(self.category.clone())
    }
}
// Second highest level holder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RotatingHolder {
    pub uuid: String,
    pub duplicates: usize,
    pub category: RotatingHolderCategory,
    pub diameter: f32,
    pub length: f32,
    pub max_rpm: u32,
    pub coolant_through: bool,
    pub tool_clamping_range: (f32, f32),
    pub taper_type: String,
    pub runout: f32,
    pub balance_grade: String,
    // New subcategory-specific fields
    pub collet_type: String,                   // For Collet
    pub weldon_flat_size: f32,                 // For EndMill (WeldonFlat)
    pub adjustable_range: (f32, f32),          // For BoringHead
    pub tension_compression_range: (f32, f32), // For Tapping (TensionCompression)
}

impl Default for RotatingHolder {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            duplicates: 1,
            category: RotatingHolderCategory::Empty,
            diameter: 1.0,
            length: 1.0,
            max_rpm: 50000,
            coolant_through: false,
            tool_clamping_range: (0.0, 1.0),
            taper_type: "".to_string(),
            runout: 1.0,
            balance_grade: "".to_string(),
            collet_type: "".to_string(),
            weldon_flat_size: 1.0,
            adjustable_range: (0.0, 1.0),
            tension_compression_range: (0.0, 1.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurningHolder {
    pub uuid: String,
    pub duplicates: usize,
    pub category: TurningHolderCategory,
    pub degree: f32,
    pub shank_height: f32,
    pub shank_width: f32,
    pub overall_length: f32,
    pub insert_size: String,
    pub handedness: Handedness,
    pub coolant_type: String,
    pub max_bore_depth: f32,
    pub min_bore_diameter: f32,
    pub max_cutting_diameter: f32,
    pub quick_change_compatible: bool,
    // New subcategory-specific fields
    pub cartridge_type: String, // For PartingGrooving (CartridgeType)
    pub thread_pitch_range: (f32, f32), // For Threading
    pub form_profile: String,   // For Form
    pub tool_post_size: String, // For QuickChangePost
}

impl Default for TurningHolder {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            duplicates: 1,
            category: TurningHolderCategory::Empty,
            degree: 15.0,
            shank_height: 10.0,
            shank_width: 10.0,
            overall_length: 10.0,
            insert_size: "".to_string(),
            handedness: Handedness::Neutral,
            coolant_type: "".to_string(),
            max_bore_depth: 10.0,
            min_bore_diameter: 1.0,
            max_cutting_diameter: 3.0,
            quick_change_compatible: false,
            cartridge_type: "".to_string(),
            thread_pitch_range: (0.0, 1.0),
            form_profile: "".to_string(),
            tool_post_size: "".to_string(),
        }
    }
}

// ------------- ROTATING HOLDERS ---------------- //
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum RotatingHolderCategory {
    #[default]
    Empty,
    Collet(ColletSubCategory),
    EndMill(EndMillSubcategory),
    ShellMill(ShellMillSubcategory),
    ShrinkFit(ShrinkFitSubcategory),
    Hydraulic(HydraulicSubcategory),
    DrillChuck(DrillChuckSubcategory),
    BoringHead(BoringHeadSubcategory),
    Tapping(TappingSubcategory),
}

impl fmt::Display for RotatingHolderCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotatingHolderCategory::Empty => write!(f, "Empty"),
            RotatingHolderCategory::Collet(x) => write!(f, "{}", x),
            RotatingHolderCategory::EndMill(x) => write!(f, "{}", x),
            RotatingHolderCategory::ShellMill(x) => write!(f, "{}", x),
            RotatingHolderCategory::ShrinkFit(x) => write!(f, "{}", x),
            RotatingHolderCategory::Hydraulic(x) => write!(f, "{}", x),
            RotatingHolderCategory::DrillChuck(x) => write!(f, "{}", x),
            RotatingHolderCategory::BoringHead(x) => write!(f, "{}", x),
            RotatingHolderCategory::Tapping(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Display for TurningHolderCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TurningHolderCategory::Empty => write!(f, "Empty"),
            TurningHolderCategory::External(x) => write!(f, "{}", x),
            TurningHolderCategory::Internal(x) => write!(f, "{}", x),
            TurningHolderCategory::PartingGrooving(x) => write!(f, "{}", x),
            TurningHolderCategory::Threading(x) => write!(f, "{}", x),
            TurningHolderCategory::Form(x) => write!(f, "{}", x),
            TurningHolderCategory::QuickChangePost(x) => write!(f, "{}", x),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ColletSubCategory {
    #[default]
    ER,
    TG,
    OZ,
}

impl fmt::Display for ColletSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColletSubCategory::ER => write!(f, "ER"),
            ColletSubCategory::TG => write!(f, "TG"),
            ColletSubCategory::OZ => write!(f, "OZ"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum EndMillSubcategory {
    #[default]
    WeldonFlat,
    MillingChuck,
}

impl fmt::Display for EndMillSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EndMillSubcategory::WeldonFlat => write!(f, "Weldon Flat"),
            EndMillSubcategory::MillingChuck => write!(f, "Milling Chuck"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ShellMillSubcategory {
    #[default]
    ShellMill,
}

impl fmt::Display for ShellMillSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellMillSubcategory::ShellMill => write!(f, "ShellMill"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ShrinkFitSubcategory {
    #[default]
    ShrinkFit,
}

impl fmt::Display for ShrinkFitSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShrinkFitSubcategory::ShrinkFit => write!(f, "ShrinkFit"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum HydraulicSubcategory {
    #[default]
    Hydraulic,
}

impl fmt::Display for HydraulicSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HydraulicSubcategory::Hydraulic => write!(f, "Hydraulic"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum DrillChuckSubcategory {
    #[default]
    DrillChuck,
}

impl fmt::Display for DrillChuckSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrillChuckSubcategory::DrillChuck => write!(f, "Drill Chuck"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum BoringHeadSubcategory {
    #[default]
    Adjustable,
    MicroAdjustable,
}

impl fmt::Display for BoringHeadSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoringHeadSubcategory::Adjustable => write!(f, "Adjustable"),
            BoringHeadSubcategory::MicroAdjustable => write!(f, "Micro Adjustable"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum TappingSubcategory {
    #[default]
    TensionCompression,
    Rigid,
}

impl fmt::Display for TappingSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TappingSubcategory::TensionCompression => write!(f, "Tension Compression"),
            TappingSubcategory::Rigid => write!(f, "Rigid"),
        }
    }
}

// ------------- TURNING HOLDERS ---------------- //
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum TurningHolderCategory {
    #[default]
    Empty,
    External(ExternalSubcategory),
    Internal(InternalSubcategory),
    PartingGrooving(PartingGroovingSubcategory),
    Threading(ThreadingSubcategory),
    Form(FormSubcategory),
    QuickChangePost(QuickChangePostSubcategory),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ExternalSubcategory {
    #[default]
    RightHand,
    LeftHand,
    Neutral,
}

impl fmt::Display for ExternalSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExternalSubcategory::RightHand => write!(f, "Right Hand"),
            ExternalSubcategory::LeftHand => write!(f, "Left Hand"),
            ExternalSubcategory::Neutral => write!(f, "Neutral"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum InternalSubcategory {
    #[default]
    BoringBar,
    InternalThreading,
}

impl fmt::Display for InternalSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalSubcategory::BoringBar => write!(f, "Boring Bar"),
            InternalSubcategory::InternalThreading => write!(f, "Internal Threading"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum PartingGroovingSubcategory {
    #[default]
    BladeType,
    CartridgeType,
}

impl fmt::Display for PartingGroovingSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartingGroovingSubcategory::BladeType => write!(f, "Blade Type"),
            PartingGroovingSubcategory::CartridgeType => write!(f, "Cartridge Type"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ThreadingSubcategory {
    #[default]
    External,
    Internal,
}

impl fmt::Display for ThreadingSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadingSubcategory::External => write!(f, "External"),
            ThreadingSubcategory::Internal => write!(f, "Internal"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum FormSubcategory {
    #[default]
    Form,
}

impl fmt::Display for FormSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormSubcategory::Form => write!(f, "Form"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum QuickChangePostSubcategory {
    #[default]
    QCTP,
}

impl fmt::Display for QuickChangePostSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuickChangePostSubcategory::QCTP => write!(f, "QCTP"),
        }
    }
}
