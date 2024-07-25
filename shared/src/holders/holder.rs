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
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum RotatingHolderCategory {
    #[default]
    Empty,
    Collet(ColletSubCategory),
    EndMill(EndMillSubCategory),
    ShellMill(ShellMillSubCategory),
    ShrinkFit(ShrinkFitSubCategory),
    Hydraulic(HydrauliScubCategory),
    DrillChuck(DrillChuckSubCategory),
    BoringHead(BoringHeadSubCategory),
    Tapping(TappingSubCategory),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ColletSubCategory {
    #[default]
    Empty,
    ER,
    TG,
    OZ,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum EndMillSubCategory {
    #[default]
    Empty,
    WeldonFlat,
    MillingChuck,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ShellMillSubCategory {
    #[default]
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ShrinkFitSubCategory {
    #[default]
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum HydrauliScubCategory {
    #[default]
    Empty,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum DrillChuckSubCategory {
    #[default]
    Empty,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum BoringHeadSubCategory {
    #[default]
    Empty,
    Adjustable,
    MicroAdjustable,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum TappingSubCategory {
    #[default]
    Empty,
    TensionCompression,
    Rigid,
}

// ------------- TURNING HOLDERS ---------------- //
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum TurningHolderCategory {
    #[default]
    Empty,
    External(ExternalSubCategory),
    Internal(InternalSubCategory),
    PartingGrooving(PartingGroovingSubCategory),
    Threading(ThreadingSubCategory),
    Form(FormSubCategory),
    QuickChangePost(QuickChangePostSubCategory),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ExternalSubCategory {
    #[default]
    Empty,
    RightHand,
    LeftHand,
    Neutral,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum InternalSubCategory {
    #[default]
    Empty,
    BoringBar,
    InternalThreading,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum PartingGroovingSubCategory {
    #[default]
    Empty,
    BladeType,
    CartridgeType,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ThreadingSubCategory {
    #[default]
    Empty,
    External,
    Internal,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum FormSubCategory {
    #[default]
    Empty,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum QuickChangePostSubCategory {
    #[default]
    Empty,
    QCTP,
}
