use serde::{Deserialize, Serialize};
use shared::{
    description::Description,
    holders::holder::{Holder, RotatingHolder, TurningHolder},
    machine::Machine,
    magazine::Magazine,
    tools::tool::{RotatingTool, Tool, TurningTool},
};

use crate::{
    app_states::{FilterState, HolderTypeSelection, SortState, ToolTypeSelection},
    widgets::{
        gripper_fixed_widget::GripperFixedCalculationData, gripper_widget::GripperCalculationData,
        unit_conversion::ConversionData,
    },
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Singletons {
    pub machine: Machine,
    pub magazine: Magazine,
    pub description: Description,
    pub should_save_user_data: bool,
    pub gripper_calculations: GripperCalculationData,
    pub gripper_fixed_calculations: GripperFixedCalculationData,
    pub rotating_tool: RotatingTool,
    pub turning_tool: TurningTool,
    pub rotating_holder: RotatingHolder,
    pub turning_holder: TurningHolder,
    pub tool_type_selection: ToolTypeSelection,
    pub holder_type_selection: HolderTypeSelection,
    pub sort_state: SortState,
    pub filter_state: FilterState,
    pub conversion_data: ConversionData,
}

impl Singletons {
    pub fn reset(&mut self) {
        self.machine = Machine::default();
        self.magazine = Magazine::default();
        self.description = Description::default();
        self.should_save_user_data = false;
        self.gripper_calculations = GripperCalculationData::default();
        self.rotating_tool = RotatingTool::default();
        self.turning_tool = TurningTool::default();
        self.rotating_holder = RotatingHolder::default();
        self.turning_holder = TurningHolder::default();
        self.tool_type_selection = ToolTypeSelection::default();
        self.holder_type_selection = HolderTypeSelection::default();
    }
}
