use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Description {
    pub text: String,
}

impl Description {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
