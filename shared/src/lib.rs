use egui::Color32;
use library::Library;
use machine::Machine;
use selection::Selections;
use serde::{Deserialize, Serialize};

pub mod custom_traits;
pub mod description;
pub mod library;
pub mod machine;
pub mod selection;
pub mod tools;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserData {
    pub favorite_color: Color32,
    pub machines: Vec<Machine>,
    pub library: Library,
    pub selections: Selections,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: Option<String>,
    pub last_login: Option<String>,
    pub user_data: UserData, // serialized as JSON
}
