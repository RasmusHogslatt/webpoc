use std::default;

use crate::holders::{
    self,
    holder::{RotatingHolder, TurningHolder},
};
use serde::{Deserialize, Serialize};

// Highest level tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tool {
    Rotating(RotatingTool),
    Turning(TurningTool),
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Rotating(RotatingTool::default())
    }
}

// Second highest level tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RotatingTool {
    pub category: RotatingToolCategory,
    pub holder: RotatingHolder,
    pub diameter: f32,
}

impl Default for RotatingTool {
    fn default() -> Self {
        Self {
            category: RotatingToolCategory::Empty,
            holder: RotatingHolder::default(),
            diameter: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurningTool {
    pub category: TurningToolCategory,
    pub holder: TurningHolder,
    pub degree: f32,
}

impl Default for TurningTool {
    fn default() -> Self {
        Self {
            category: TurningToolCategory::Empty,
            holder: TurningHolder::default(),
            degree: 15.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RotatingToolCategory {
    Empty,
    BallNoseMill,
    BoringTool,
    ChamferMill,
    DoveTailCutter,
    DrillBit,
    EndMill,
    FaceMill,
    Reamer,
    SlotDrill,
    ThreadMill,
    TSlotCutter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TurningToolCategory {
    Empty,
    TurningTool,
    FacingTool,
    BoringBar,
    ThreadingTool,
    GroovingPartingTool,
    FormTool,
}
