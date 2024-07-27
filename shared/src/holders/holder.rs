use serde::{Deserialize, Serialize};

use crate::tools::tool::Handedness;

// Highest level holder
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Holder {
    Rotating(RotatingHolder),
    Turning(TurningHolder),
}

impl Default for Holder {
    fn default() -> Self {
        Holder::Rotating(RotatingHolder::default())
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

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ColletSubCategory {
    #[default]
    Empty,
    ER,
    TG,
    OZ,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum EndMillSubcategory {
    #[default]
    Empty,
    WeldonFlat,
    MillingChuck,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ShellMillSubcategory {
    #[default]
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ShrinkFitSubcategory {
    #[default]
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum HydraulicSubcategory {
    #[default]
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum DrillChuckSubcategory {
    #[default]
    Empty,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum BoringHeadSubcategory {
    #[default]
    Empty,
    Adjustable,
    MicroAdjustable,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum TappingSubcategory {
    #[default]
    Empty,
    TensionCompression,
    Rigid,
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

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum InternalSubcategory {
    #[default]
    Empty,
    BoringBar,
    InternalThreading,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum PartingGroovingSubcategory {
    #[default]
    Empty,
    BladeType,
    CartridgeType,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum ThreadingSubcategory {
    #[default]
    Empty,
    External,
    Internal,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum FormSubcategory {
    #[default]
    Empty,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum QuickChangePostSubcategory {
    #[default]
    Empty,
    QCTP,
}
