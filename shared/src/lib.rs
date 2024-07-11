use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserData {
    pub favorite_color: Color32,
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
