use reqwest::Client;
use shared::User;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    username: String,
    password: String,
    login_status: Option<bool>,
    client: Client,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            login_status: None,
            client: Client::new(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    async fn verify_user(&self) -> Result<bool, reqwest::Error> {
        let user = User {
            username: self.username.clone(),
            password: self.password.clone(),
        };
        let response = self
            .client
            .post("http://localhost:8080/api/login")
            .json(&user)
            .send()
            .await?;

        Ok(response.status().is_success())
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
                let future = self.verify_user();
                let ctx = ctx.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match future.await {
                        Ok(is_valid) => {
                            let mut app = TemplateApp::default(); // This is a hack, you might want to use a better state management solution
                            app.login_status = Some(is_valid);
                            ctx.request_repaint();
                        }
                        Err(_) => {
                            let mut app = TemplateApp::default();
                            app.login_status = Some(false);
                            ctx.request_repaint();
                        }
                    }
                });
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
