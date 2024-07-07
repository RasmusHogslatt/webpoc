use crate::db;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    username: String,
    password: String,
    #[serde(skip)]
    login_status: Option<bool>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            password: "".to_owned(),
            login_status: None,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize the database
        db::init_db().expect("Failed to initialize database");

        // Add a default user (you can remove this in production)
        let _ = db::add_user("admin", "admin");
        let _ = db::add_user("user1", "user1");
        let _ = db::add_user("user2", "user2");

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sign in page");

            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(&mut self.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.text_edit_singleline(&mut self.password);
            });
            if ui.button("Sign in").clicked() {
                match db::verify_user(&self.username, &self.password) {
                    Ok(is_valid) => self.login_status = Some(is_valid),
                    Err(_) => self.login_status = Some(false),
                }
            }
            if let Some(status) = self.login_status {
                if status {
                    ui.label("Login successful!");
                } else {
                    ui.label("Login failed!");
                }
            }
        });
    }
}
