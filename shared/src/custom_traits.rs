use egui::Ui;

use crate::{
    holders::holder::{RotatingHolderCategory, TurningHolderCategory},
    tools::tool::{RotatingToolCategory, TurningToolCategory},
};

pub trait GetName {
    fn get_name(&self) -> String;
}

pub trait GetRotatingToolCategory {
    fn get_rotating_tool_category(&self) -> Option<RotatingToolCategory>;
}

pub trait GetTurningToolCategory {
    fn get_turning_tool_category(&self) -> Option<TurningToolCategory>;
}

pub trait GetRotatingHolderCategory {
    fn get_rotating_holder_category(&self) -> Option<RotatingHolderCategory>;
}

pub trait GetTurningHolderCategory {
    fn get_turning_holder_category(&self) -> Option<TurningHolderCategory>;
}

pub trait GetDiameter {
    fn get_diameter(&self) -> f32;
}

pub trait GetDegree {
    fn get_degree(&self) -> f32;
}

pub trait GetHolderType {
    fn is_rotating(&self) -> bool;
    fn is_turning(&self) -> bool;
}

pub trait GetToolType {
    fn is_rotating(&self) -> bool;
    fn is_turning(&self) -> bool;
}

pub trait UiDisplay {
    fn display(&self, ui: &mut Ui);
}

// Returns true if holder should be entirely deleted
pub trait DeleteHolder {
    fn delete_holder(&mut self) -> bool;
}

pub trait GetUuid {
    fn get_uuid(&self) -> String;
}

pub trait AddHolderCopy {
    fn add_copy(&mut self);
}
