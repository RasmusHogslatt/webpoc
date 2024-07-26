use core::fmt;
use std::{default, fmt::write};

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
    pub cutting_diameter: f32,
    pub connection_diameter: f32,
    pub usable_length: f32,
    pub achievable_hole_tolerance: f32,
    pub functional_length: f32,
    pub weight_of_tool: f32,
    pub max_rpm: u32,          // RPM
    pub coolant_pressure: u32, // BAR
}

impl Default for RotatingTool {
    fn default() -> Self {
        Self {
            category: RotatingToolCategory::Empty,
            cutting_diameter: 1.0,
            connection_diameter: 5.0,
            usable_length: 10.0,
            achievable_hole_tolerance: 0.01,
            functional_length: 10.0,
            weight_of_tool: 10.0,
            max_rpm: 50000,
            coolant_pressure: 20,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurningTool {
    pub category: TurningToolCategory,
    pub lead_angle: f32, // DEGREE
    pub handedness: Handedness,
    pub cutting_edge_angle: f32, // DEGREE
    pub insert_type: String,
    pub maximum_ramping_angle: f32,
    pub minimum_bore_diameter: f32,
    pub workpiece_side_body_angle: f32, // DEGREE
    pub cutting_depth_maximum: f32,
    pub machine_side_body_angle: f32, // DEGREE
    pub minimum_overhang: f32,
    pub maximum_overhang: f32,
    pub usable_length: f32,
    pub body_length: f32,
    pub body_diameter: f32,
    pub functional_diameter: f32,
    pub peripheral_effective_cutting: f32,
    pub connection_diameter: f32,
    pub maximum_rpm: u32,
    pub tool_weight: f32,
}

impl Default for TurningTool {
    fn default() -> Self {
        Self {
            category: TurningToolCategory::Empty,
            lead_angle: 0.0,
            handedness: Handedness::default(),
            cutting_edge_angle: 0.0,
            insert_type: "".to_string(),
            maximum_ramping_angle: 90.0,
            minimum_bore_diameter: 0.1,
            workpiece_side_body_angle: 0.0,
            cutting_depth_maximum: 1.0,
            machine_side_body_angle: 0.0,
            minimum_overhang: 0.0,
            maximum_overhang: 10.0,
            usable_length: 5.0,
            body_length: 10.0,
            body_diameter: 5.0,
            functional_diameter: 10.0,
            peripheral_effective_cutting: 1.0,
            connection_diameter: 5.0,
            maximum_rpm: 50000,
            tool_weight: 20.0, // GRAMS
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum Handedness {
    #[default]
    Neutral,
    Left,
    Right,
}

impl fmt::Display for Handedness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Handedness::Neutral => write!(f, "Neutral"),
            Handedness::Left => write!(f, "Left"),
            Handedness::Right => write!(f, "Right"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TurningToolCategory {
    Empty,
    InternalTurningTool,
    ExternalTurningTool,
    FacingTool,
    BoringBar,
    ThreadingTool,
    GroovingPartingTool,
    FormTool,
}
