use serde::{Deserialize, Serialize};

pub const LIBRARY_COLUMN_WIDTH: f32 = 220.0;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct Settings {
    pub dark_mode: bool,
    pub show_password: bool,
}
