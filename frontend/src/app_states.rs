use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppState {
    WelcomePage,
    SignIn,
    SignUp,
    Application,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WidgetState {
    Default,
    AddMachine,
    DeleteMachine,
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
}
