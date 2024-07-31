use crate::singletons::Singletons;
use egui::{Response, Ui, Widget};
use shared::User;

pub struct MagazineWidget<'a> {
    user: &'a mut User,
    singletons: &'a mut Singletons,
}

impl<'a> MagazineWidget<'a> {
    pub fn new(user: &'a mut User, singletons: &'a mut Singletons) -> Self {
        Self { user, singletons }
    }
}

impl<'a> Widget for MagazineWidget<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.centered_and_justified(|ui| ui.label("Magazine Widget"))
            .response
    }
}
