use crate::tools::rotating::*;
use crate::tools::turning::*;

// Highest level tool
pub enum Tool {
    Rotating(RotatingTool),
    Turning(TurningTool),
}

// Second highest level tool
pub enum RotatingTool {
    Drill(DrillType),
    Mill(MillType),
    TapType(TapType),
    BroachType(BroachType),
    HolderType(HolderType),
}

pub enum TurningTool {
    Lathe(LatheType),
}
