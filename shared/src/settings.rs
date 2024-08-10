use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct Settings {
    pub dark_mode: bool,
    pub show_password: bool,
    pub color1: Color32,
    pub color2: Color32,
    pub color3: Color32,
}
