use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct Settings {
    pub dark_mode: bool,
    pub show_password: bool,
}
