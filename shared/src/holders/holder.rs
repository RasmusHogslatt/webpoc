use serde::{Deserialize, Serialize};

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
}

impl Default for RotatingHolder {
    fn default() -> Self {
        Self {
            category: RotatingHolderCategory::Empty,
            diameter: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurningHolder {
    pub category: TurningHolderCategory,
    pub degree: f32,
}

impl Default for TurningHolder {
    fn default() -> Self {
        Self {
            category: TurningHolderCategory::Empty,
            degree: 15.0,
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
