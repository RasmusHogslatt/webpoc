use crate::tools::rotating::*;
use crate::tools::turning::*;
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
pub enum RotatingTool {
    Drill(DrillType),
    Mill(MillType),
    TapType(TapType),
    BroachType(BroachType),
    HolderType(HolderType),
}

impl Default for RotatingTool {
    fn default() -> Self {
        RotatingTool::Drill(DrillType::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TurningTool {
    Lathe(LatheType),
}

impl Default for TurningTool {
    fn default() -> Self {
        TurningTool::Lathe(LatheType::default())
    }
}
