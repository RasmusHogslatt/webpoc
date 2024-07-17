use serde::{Deserialize, Serialize};

use crate::{description::Description, tools::tool::Tool};

pub type Content = (usize, Option<Tool>, Description);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Magazine {
    pub name: String,
    pub index_in_machine: usize,
    pub content: Vec<Content>,
    pub capacity: usize,
}

impl Magazine {
    pub fn new(index_in_machine: usize, capacity: usize) -> Self {
        let name = format!("Magazine {}", index_in_machine + 1);
        let mut content = vec![];
        for index in 0..capacity {
            content.push((index, None, Description::new("...".to_string())));
        }
        Self {
            name: name.clone(),
            index_in_machine,
            content: content.clone(),
            capacity,
        }
    }
}
