use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum DrillType {
    #[default]
    BoringHead,
    CarbideDrill,
    HSSDrill,
    Pilot,
    StudHole,
}
