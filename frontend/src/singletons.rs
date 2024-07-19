use serde::{Deserialize, Serialize};
use shared::{description::Description, machine::Machine, magazine::Magazine};

use crate::widgets::{
    gripper_fixed_widget::GripperFixedCalculationData, gripper_widget::GripperCalculationData,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Singletons {
    pub machine: Machine,
    pub magazine: Magazine,
    pub description: Description,
    pub should_save_user_data: bool,
    pub gripper_calculations: GripperCalculationData,
    pub gripper_fixed_calculations: GripperFixedCalculationData,
}

impl Singletons {
    pub fn reset(&mut self) {
        self.machine = Machine::default();
        self.magazine = Magazine::default();
        self.description = Description::default();
        self.should_save_user_data = false;
        self.gripper_calculations = GripperCalculationData::default();
    }
}
