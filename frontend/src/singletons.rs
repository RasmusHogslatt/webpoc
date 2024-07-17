use serde::{Deserialize, Serialize};
use shared::{description::Description, machine::Machine, magazine::Magazine};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Singletons {
    pub machine: Machine,
    pub magazine: Magazine,
    pub description: Description,
    pub should_save_user_data: bool,
}

impl Singletons {
    pub fn Reset(&mut self) {
        self.machine = Machine::default();
        self.magazine = Magazine::default();
        self.description = Description::default();
        self.should_save_user_data = false;
    }
}
