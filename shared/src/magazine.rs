use serde::{Deserialize, Serialize};

use crate::{description::Description, tools::tool::Tool};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Magazine {
    pub name: String,
    pub index_in_machine: usize,
    pub content: Vec<(Option<Tool>, Description)>,
}
