use serde::{Deserialize, Serialize};
use shared::{description::Description, machine::Machine, magazine::Magazine};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Singletons {
    pub machine: Machine,
    pub magazine: Magazine,
    pub description: Description,
    pub should_save_user_data: bool,
}
