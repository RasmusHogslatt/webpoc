use std::fmt;

use egui::Ui;
use serde::{Deserialize, Serialize};

use crate::{
    custom_traits::{
        GetHolderType, GetRotatingHolderCategory, GetSlot, GetTurningHolderCategory, SetSlot,
        UiDisplay,
    },
    tools::tool::Handedness,
};

// Highest level holder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Holder {
    Rotating(RotatingHolder),
    Turning(TurningHolder),
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
            ui.vertical(|ui| {
                ui.label(format!("Category: {}", self.category));
                ui.label(format!("Diameter: {:.2} mm", self.diameter));
                ui.label(format!("Length: {:.2} mm", self.length));

                match &self.category {
                    RotatingHolderCategory::Empty => {}
                    RotatingHolderCategory::Collet(subcategory) => {
                        ui.label(format!("Collet Type: {}", subcategory));
                    }
                    RotatingHolderCategory::EndMill(subcategory) => {
                        ui.label(format!("End Mill Type: {}", subcategory));
                    }
                    RotatingHolderCategory::ShellMill(subcategory) => {
                        ui.label(format!("Shell Mill Type: {}", subcategory));
                    }
                    RotatingHolderCategory::ShrinkFit(subcategory) => {
                        ui.label(format!("Shrink Fit Type: {}", subcategory));
                    }
                    RotatingHolderCategory::Hydraulic(subcategory) => {
                        ui.label(format!("Hydraulic Type: {}", subcategory));
                    }
                    RotatingHolderCategory::DrillChuck(subcategory) => {
                        ui.label(format!("Drill Chuck Type: {}", subcategory));
                    }
                    RotatingHolderCategory::BoringHead(subcategory) => {
                        ui.label(format!("Boring Head Type: {}", subcategory));
                    }
                    RotatingHolderCategory::Tapping(subcategory) => {
                        ui.label(format!("Tapping Type: {}", subcategory));
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
            ui.vertical(|ui| {
                ui.label(format!("Category: {}", self.category));
                ui.label(format!(
                    "Shank: {:.2}x{:.2} mm",
                    self.shank_width, self.shank_height
                ));
                ui.label(format!("Overall Length: {:.2} mm", self.overall_length));

                match &self.category {
                    TurningHolderCategory::Empty => {}
                    TurningHolderCategory::External(subcategory) => {
                        ui.label(format!("External Type: {}", subcategory));
                    }
                    TurningHolderCategory::Internal(subcategory) => {
                        ui.label(format!("Internal Type: {}", subcategory));
                    }
                    TurningHolderCategory::PartingGrooving(subcategory) => {
                        ui.label(format!("Parting/Grooving Type: {}", subcategory));
                    }
                    TurningHolderCategory::Threading(subcategory) => {
                        ui.label(format!("Threading Type: {}", subcategory));
                    }
                    TurningHolderCategory::Form(subcategory) => {
                        ui.label(format!("Form Type: {}", subcategory));
                    }
                    TurningHolderCategory::QuickChangePost(subcategory) => {
                        ui.label(format!("Quick Change Post Type: {}", subcategory));
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

impl GetSlot for Holder {
    fn get_library_slot(&self) -> Option<usize> {
        match self {
            Holder::Rotating(item) => item.get_library_slot(),
            Holder::Turning(item) => item.get_library_slot(),
        }
    }
    fn get_machine_slot(&self) -> Option<usize> {
        match self {
            Holder::Rotating(item) => item.get_machine_slot(),
            Holder::Turning(item) => item.get_machine_slot(),
        }
    }
}

impl GetSlot for RotatingHolder {
    fn get_library_slot(&self) -> Option<usize> {
        self.library_slot
    }
    fn get_machine_slot(&self) -> Option<usize> {
        self.machine_slot
    }
}

impl GetSlot for TurningHolder {
    fn get_library_slot(&self) -> Option<usize> {
        self.library_slot
    }
    fn get_machine_slot(&self) -> Option<usize> {
        self.machine_slot
    }
}

impl SetSlot for Holder {
    fn set_library_slot(&mut self, index: Option<usize>) {
        match self {
            Holder::Rotating(item) => item.set_library_slot(index),
            Holder::Turning(item) => item.set_library_slot(index),
        }
    }
    fn set_machine_slot(&mut self, index: Option<usize>) {
        match self {
            Holder::Rotating(item) => item.set_machine_slot(index),
            Holder::Turning(item) => item.set_machine_slot(index),
        }
    }
}

impl SetSlot for RotatingHolder {
    fn set_library_slot(&mut self, index: Option<usize>) {
        self.library_slot = index;
    }
    fn set_machine_slot(&mut self, index: Option<usize>) {
        self.machine_slot = index;
    }
}

impl SetSlot for TurningHolder {
    fn set_library_slot(&mut self, index: Option<usize>) {
        self.library_slot = index;
    }
    fn set_machine_slot(&mut self, index: Option<usize>) {
        self.machine_slot = index;
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
    pub library_slot: Option<usize>,
    pub machine_slot: Option<usize>,
}

impl Default for RotatingHolder {
    fn default() -> Self {
        Self {
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
            library_slot: None,
            machine_slot: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurningHolder {
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
    pub library_slot: Option<usize>,
    pub machine_slot: Option<usize>,
}

impl Default for TurningHolder {
    fn default() -> Self {
        Self {
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
            library_slot: None,
            machine_slot: None,
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
    Empty,
    ER,
    TG,
    OZ,
}

impl fmt::Display for ColletSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColletSubCategory::Empty => write!(f, "Empty"),
            ColletSubCategory::ER => write!(f, "ER"),
            ColletSubCategory::TG => write!(f, "TG"),
            ColletSubCategory::OZ => write!(f, "OZ"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum EndMillSubcategory {
    #[default]
    Empty,
    WeldonFlat,
    MillingChuck,
}

impl fmt::Display for EndMillSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EndMillSubcategory::Empty => write!(f, "Empty"),
            EndMillSubcategory::WeldonFlat => write!(f, "Weldon Flat"),
            EndMillSubcategory::MillingChuck => write!(f, "Milling Chuck"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ShellMillSubcategory {
    #[default]
    Empty,
}

impl fmt::Display for ShellMillSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShellMillSubcategory::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ShrinkFitSubcategory {
    #[default]
    Empty,
}

impl fmt::Display for ShrinkFitSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShrinkFitSubcategory::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum HydraulicSubcategory {
    #[default]
    Empty,
}

impl fmt::Display for HydraulicSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HydraulicSubcategory::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum DrillChuckSubcategory {
    #[default]
    Empty,
}

impl fmt::Display for DrillChuckSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DrillChuckSubcategory::Empty => write!(f, "Empty"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum BoringHeadSubcategory {
    #[default]
    Empty,
    Adjustable,
    MicroAdjustable,
}

impl fmt::Display for BoringHeadSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoringHeadSubcategory::Empty => write!(f, "Empty"),
            BoringHeadSubcategory::Adjustable => write!(f, "Adjustable"),
            BoringHeadSubcategory::MicroAdjustable => write!(f, "Micro Adjustable"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum TappingSubcategory {
    #[default]
    Empty,
    TensionCompression,
    Rigid,
}

impl fmt::Display for TappingSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TappingSubcategory::Empty => write!(f, "Empty"),
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
    Empty,
    RightHand,
    LeftHand,
    Neutral,
}

impl fmt::Display for ExternalSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExternalSubcategory::Empty => write!(f, "Empty"),
            ExternalSubcategory::RightHand => write!(f, "Right Hand"),
            ExternalSubcategory::LeftHand => write!(f, "Left Hand"),
            ExternalSubcategory::Neutral => write!(f, "Neutral"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum InternalSubcategory {
    #[default]
    Empty,
    BoringBar,
    InternalThreading,
}

impl fmt::Display for InternalSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalSubcategory::Empty => write!(f, "Empty"),
            InternalSubcategory::BoringBar => write!(f, "Boring Bar"),
            InternalSubcategory::InternalThreading => write!(f, "Internal Threading"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum PartingGroovingSubcategory {
    #[default]
    Empty,
    BladeType,
    CartridgeType,
}

impl fmt::Display for PartingGroovingSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartingGroovingSubcategory::Empty => write!(f, "Empty"),
            PartingGroovingSubcategory::BladeType => write!(f, "Blade Type"),
            PartingGroovingSubcategory::CartridgeType => write!(f, "Cartridge Type"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ThreadingSubcategory {
    #[default]
    Empty,
    External,
    Internal,
}

impl fmt::Display for ThreadingSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadingSubcategory::Empty => write!(f, "Empty"),
            ThreadingSubcategory::External => write!(f, "External"),
            ThreadingSubcategory::Internal => write!(f, "Internal"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum FormSubcategory {
    #[default]
    Empty,
}

impl fmt::Display for FormSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormSubcategory::Empty => write!(f, "Empty"),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum QuickChangePostSubcategory {
    #[default]
    Empty,
    QCTP,
}

impl fmt::Display for QuickChangePostSubcategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuickChangePostSubcategory::Empty => write!(f, "Empty"),
            QuickChangePostSubcategory::QCTP => write!(f, "QCTP"),
        }
    }
}
