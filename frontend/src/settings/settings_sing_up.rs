#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SettingsSignUp {
    pub show_password: bool,
}

impl SettingsSignUp {
    pub fn default() -> Self {
        Self {
            show_password: false,
        }
    }
}
