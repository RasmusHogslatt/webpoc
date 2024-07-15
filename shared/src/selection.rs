use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Selections {
    pub selected_machine: Option<usize>,
}
