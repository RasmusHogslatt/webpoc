use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppState {
    WelcomePage,
    SignIn,
    SignUp,
    Application,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CentralViewState {
    #[default]
    Magazine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SortState {
    #[default]
    Index,
    Diameter,
    Degree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FilterState {
    #[default]
    ShowAll,
    RotatingToolCategory,
    RotatingHolderCategory,
    TurningToolCategory,
    TurningHolderCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ToolTypeSelection {
    #[default]
    Rotating,
    Turning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HolderTypeSelection {
    #[default]
    Rotating,
    Turning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetState {
    Default,
    AddMachine,
    DeleteMachine,
    EditMachine,
    Settings,
    GripperCalculation,
    GripperFixedCalculation,
    AddTool,
    AddHolder,
}

impl Default for WidgetState {
    fn default() -> Self {
        WidgetState::Default
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenWindows {
    pub add_machine_window_open: bool,
    pub delete_machine_window_open: bool,
    pub edit_machine_window_open: bool,
    pub gripper_window_open: bool,
    pub gripper_fixed_window_open: bool,
    pub settings_window_open: bool,
    pub add_tool_window: bool,
    pub add_holder_window: bool,
}

impl OpenWindows {
    pub fn reset(&mut self) {
        self.add_holder_window = false;
        self.add_machine_window_open = false;
        self.add_tool_window = false;
        self.delete_machine_window_open = false;
        self.edit_machine_window_open = false;
        self.gripper_fixed_window_open = false;
        self.gripper_window_open = false;
        self.settings_window_open = false;
    }
}
