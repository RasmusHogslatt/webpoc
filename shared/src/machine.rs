use crate::custom_traits::*;
use crate::description::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Machine {
    pub name: String,
    pub manufacturer: String,
    pub model: String,
    pub description: Description,
}

impl GetName for Machine {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Machine {
    pub fn new(
        name: String,
        manufacturer: String,
        model: String,
        description: Description,
    ) -> Self {
        Self {
            name,
            manufacturer,
            model,
            description,
        }
    }
}
