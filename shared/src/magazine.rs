use serde::{Deserialize, Serialize};

use crate::{description::Description, holders::holder::Holder, tools::tool::Tool};

pub type Content = (
    Option<String>,
    Option<Tool>,
    Option<Holder>,
    Option<f32>,
    Description,
);

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
            content.push((None, None, None, None, Description::new("...".to_string())));
        }
        Self {
            name: name.clone(),
            index_in_machine,
            content: content.clone(),
            capacity,
        }
    }
}
