use core::fmt;
use egui::Ui;
use enum_iterator::Sequence;
use uuid::Uuid;

use crate::custom_traits::{
    GetDegree, GetDiameter, GetRotatingToolCategory, GetToolType, GetTurningToolCategory, GetUuid,
    UiDisplay,
};
use serde::{Deserialize, Serialize};

// Highest level tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tool {
    Rotating(RotatingTool),
    Turning(TurningTool),
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Rotating(RotatingTool::default())
    }
}

impl GetUuid for Tool {
    fn get_uuid(&self) -> String {
        match self {
            Tool::Rotating(x) => x.get_uuid(),
            Tool::Turning(x) => x.get_uuid(),
        }
    }
}
impl GetUuid for RotatingTool {
    fn get_uuid(&self) -> String {
        self.uuid.to_string()
    }
}

impl GetUuid for TurningTool {
    fn get_uuid(&self) -> String {
        self.uuid.to_string()
    }
}

fn rotating_tool_hover_ui(ui: &mut Ui, rotating_tool: &RotatingTool) {
    egui::Grid::new("rotating_tool_info")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Cutting Diameter:");
            ui.label(format!("{:.2} mm", rotating_tool.cutting_diameter));
            ui.end_row();
            ui.label("Connection Diameter:");
            ui.label(format!("{:.2} mm", rotating_tool.connection_diameter));
            ui.end_row();
            ui.label("Usable Length:");
            ui.label(format!("{:.2} mm", rotating_tool.usable_length));
            ui.end_row();
            ui.label("Achievable Hole Tolerance:");
            ui.label(format!("{:.3} mm", rotating_tool.achievable_hole_tolerance));
            ui.end_row();
            ui.label("Functional Length:");
            ui.label(format!("{:.2} mm", rotating_tool.functional_length));
            ui.end_row();
            ui.label("Weight of Tool:");
            ui.label(format!("{:.2} g", rotating_tool.weight_of_tool));
            ui.end_row();
            ui.label("Max RPM:");
            ui.label(format!("{} RPM", rotating_tool.max_rpm));
            ui.end_row();
            ui.label("Coolant Pressure:");
            ui.label(format!("{} BAR", rotating_tool.coolant_pressure));
            ui.end_row();
        });
}

fn turning_tool_hover_ui(ui: &mut Ui, turning_tool: &TurningTool) {
    egui::Grid::new("turning_tool_info")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Lead Angle:");
            ui.label(format!("{:.2}°", turning_tool.lead_angle));
            ui.end_row();
            ui.label("Handedness:");
            ui.label(format!("{}", turning_tool.handedness));
            ui.end_row();
            ui.label("Cutting Edge Angle:");
            ui.label(format!("{:.2}°", turning_tool.cutting_edge_angle));
            ui.end_row();
            ui.label("Insert Type:");
            ui.label(&turning_tool.insert_type);
            ui.end_row();
            ui.label("Maximum Ramping Angle:");
            ui.label(format!("{:.2}°", turning_tool.maximum_ramping_angle));
            ui.end_row();
            ui.label("Minimum Bore Diameter:");
            ui.label(format!("{:.2} mm", turning_tool.minimum_bore_diameter));
            ui.end_row();
            ui.label("Workpiece Side Body Angle:");
            ui.label(format!("{:.2}°", turning_tool.workpiece_side_body_angle));
            ui.end_row();
            ui.label("Cutting Depth Maximum:");
            ui.label(format!("{:.2} mm", turning_tool.cutting_depth_maximum));
            ui.end_row();
            ui.label("Machine Side Body Angle:");
            ui.label(format!("{:.2}°", turning_tool.machine_side_body_angle));
            ui.end_row();
            ui.label("Minimum Overhang:");
            ui.label(format!("{:.2} mm", turning_tool.minimum_overhang));
            ui.end_row();
            ui.label("Maximum Overhang:");
            ui.label(format!("{:.2} mm", turning_tool.maximum_overhang));
            ui.end_row();
            ui.label("Usable Length:");
            ui.label(format!("{:.2} mm", turning_tool.usable_length));
            ui.end_row();
            ui.label("Body Length:");
            ui.label(format!("{:.2} mm", turning_tool.body_length));
            ui.end_row();
            ui.label("Body Diameter:");
            ui.label(format!("{:.2} mm", turning_tool.body_diameter));
            ui.end_row();
            ui.label("Functional Diameter:");
            ui.label(format!("{:.2} mm", turning_tool.functional_diameter));
            ui.end_row();
            ui.label("Peripheral Effective Cutting:");
            ui.label(format!(
                "{:.2} mm",
                turning_tool.peripheral_effective_cutting
            ));
            ui.end_row();
            ui.label("Connection Diameter:");
            ui.label(format!("{:.2} mm", turning_tool.connection_diameter));
            ui.end_row();
            ui.label("Maximum RPM:");
            ui.label(format!("{} RPM", turning_tool.maximum_rpm));
            ui.end_row();
            ui.label("Tool Weight:");
            ui.label(format!("{:.2} g", turning_tool.tool_weight));
            ui.end_row();
        });
}

impl UiDisplay for RotatingTool {
    fn display(&self, ui: &mut egui::Ui) {
        let response = ui.group(|ui| {
            // Create a unique ID for each grid based on the RotatingTool's properties
            let grid_id = egui::Id::new(format!("rotating_tool_grid_{}", self.uuid));

            egui::Grid::new(grid_id)
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Category:");
                    ui.label(format!("{}", self.category));
                    ui.end_row();

                    ui.label("Cutting Diameter:");
                    ui.label(format!("{:.2} mm", self.cutting_diameter));
                    ui.end_row();

                    ui.label("Usable Length:");
                    ui.label(format!("{:.2} mm", self.usable_length));
                    ui.end_row();

                    match &self.category {
                        RotatingToolCategory::Empty => {}
                        RotatingToolCategory::BallNoseMill => {
                            ui.label("Type:");
                            ui.label("Ball Nose Mill");
                            ui.end_row();
                        }
                        RotatingToolCategory::BoringTool => {
                            ui.label("Type:");
                            ui.label("Boring Tool");
                            ui.end_row();
                        }
                        RotatingToolCategory::ChamferMill => {
                            ui.label("Type:");
                            ui.label("Chamfer Mill");
                            ui.end_row();
                        }
                        RotatingToolCategory::DoveTailCutter => {
                            ui.label("Type:");
                            ui.label("Dove Tail Cutter");
                            ui.end_row();
                        }
                        RotatingToolCategory::DrillBit => {
                            ui.label("Type:");
                            ui.label("Drill Bit");
                            ui.end_row();
                        }
                        RotatingToolCategory::EndMill => {
                            ui.label("Type:");
                            ui.label("End Mill");
                            ui.end_row();
                        }
                        RotatingToolCategory::FaceMill => {
                            ui.label("Type:");
                            ui.label("Face Mill");
                            ui.end_row();
                        }
                        RotatingToolCategory::Reamer => {
                            ui.label("Type:");
                            ui.label("Reamer");
                            ui.end_row();
                        }
                        RotatingToolCategory::SlotDrill => {
                            ui.label("Type:");
                            ui.label("Slot Drill");
                            ui.end_row();
                        }
                        RotatingToolCategory::ThreadMill => {
                            ui.label("Type:");
                            ui.label("Thread Mill");
                            ui.end_row();
                        }
                        RotatingToolCategory::TSlotCutter => {
                            ui.label("Type:");
                            ui.label("T-Slot Cutter");
                            ui.end_row();
                        }
                    }
                });
        });

        // Add hover effect for all cases
        response.response.on_hover_ui(|ui| {
            rotating_tool_hover_ui(ui, self);
        });
    }
}
impl UiDisplay for TurningTool {
    fn display(&self, ui: &mut egui::Ui) {
        let response = ui.group(|ui| {
            // Create a unique ID for each grid based on the TurningTool's properties
            let grid_id = egui::Id::new(format!("turning_tool_grid_{}", self.uuid));

            egui::Grid::new(grid_id)
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Category:");
                    ui.label(format!("{}", self.category));
                    ui.end_row();

                    ui.label("Insert Type:");
                    ui.label(format!("{}", self.insert_type));
                    ui.end_row();

                    ui.label("Handedness:");
                    ui.label(format!("{}", self.handedness));
                    ui.end_row();

                    match &self.category {
                        TurningToolCategory::Empty => {}
                        TurningToolCategory::InternalTurningTool => {
                            ui.label("Type:");
                            ui.label("Internal Turning Tool");
                            ui.end_row();
                        }
                        TurningToolCategory::ExternalTurningTool => {
                            ui.label("Type:");
                            ui.label("External Turning Tool");
                            ui.end_row();
                        }
                        TurningToolCategory::FacingTool => {
                            ui.label("Type:");
                            ui.label("Facing Tool");
                            ui.end_row();
                        }
                        TurningToolCategory::BoringBar => {
                            ui.label("Type:");
                            ui.label("Boring Bar");
                            ui.end_row();
                        }
                        TurningToolCategory::ThreadingTool => {
                            ui.label("Type:");
                            ui.label("Threading Tool");
                            ui.end_row();
                        }
                        TurningToolCategory::GroovingPartingTool => {
                            ui.label("Type:");
                            ui.label("Grooving/Parting Tool");
                            ui.end_row();
                        }
                        TurningToolCategory::FormTool => {
                            ui.label("Type:");
                            ui.label("Form Tool");
                            ui.end_row();
                        }
                    }
                });
        });

        // Add hover effect for all cases
        response.response.on_hover_ui(|ui| {
            turning_tool_hover_ui(ui, self);
        });
    }
}

impl UiDisplay for Tool {
    fn display(&self, ui: &mut egui::Ui) {
        match self {
            Tool::Rotating(rotating_tool) => rotating_tool.display(ui),
            Tool::Turning(turning_tool) => turning_tool.display(ui),
        }
    }
}

impl GetToolType for Tool {
    fn is_rotating(&self) -> bool {
        match self {
            Tool::Rotating(_) => true,
            Tool::Turning(_) => false,
        }
    }

    fn is_turning(&self) -> bool {
        match self {
            Tool::Rotating(_) => false,
            Tool::Turning(_) => true,
        }
    }
}

impl GetRotatingToolCategory for Tool {
    fn get_rotating_tool_category(&self) -> Option<RotatingToolCategory> {
        match self {
            Tool::Rotating(item) => item.get_rotating_tool_category(),
            Tool::Turning(_) => None,
        }
    }
}

impl GetRotatingToolCategory for RotatingTool {
    fn get_rotating_tool_category(&self) -> Option<RotatingToolCategory> {
        Some(self.category.clone())
    }
}

impl GetTurningToolCategory for Tool {
    fn get_turning_tool_category(&self) -> Option<TurningToolCategory> {
        match self {
            Tool::Rotating(_) => None,
            Tool::Turning(item) => item.get_turning_tool_category(),
        }
    }
}

impl GetTurningToolCategory for TurningTool {
    fn get_turning_tool_category(&self) -> Option<TurningToolCategory> {
        Some(self.category.clone())
    }
}

impl GetDiameter for Tool {
    fn get_diameter(&self) -> f32 {
        match self {
            Tool::Rotating(item) => item.get_diameter(),
            Tool::Turning(item) => item.get_diameter(),
        }
    }
}

impl GetDiameter for RotatingTool {
    fn get_diameter(&self) -> f32 {
        self.cutting_diameter
    }
}

impl GetDiameter for TurningTool {
    fn get_diameter(&self) -> f32 {
        0.0
    }
}

impl GetDegree for Tool {
    fn get_degree(&self) -> f32 {
        match self {
            Tool::Rotating(item) => item.get_degree(),
            Tool::Turning(item) => item.get_degree(),
        }
    }
}

impl GetDegree for RotatingTool {
    fn get_degree(&self) -> f32 {
        0.0
    }
}

impl GetDegree for TurningTool {
    fn get_degree(&self) -> f32 {
        self.cutting_edge_angle
    }
}

// Second highest level tool
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RotatingTool {
    pub uuid: String,
    pub category: RotatingToolCategory,
    pub cutting_diameter: f32,
    pub connection_diameter: f32,
    pub usable_length: f32,
    pub achievable_hole_tolerance: f32,
    pub functional_length: f32,
    pub weight_of_tool: f32,
    pub max_rpm: u32,          // RPM
    pub coolant_pressure: u32, // BAR
}

impl Default for RotatingTool {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            category: RotatingToolCategory::Empty,
            cutting_diameter: 1.0,
            connection_diameter: 5.0,
            usable_length: 10.0,
            achievable_hole_tolerance: 0.01,
            functional_length: 10.0,
            weight_of_tool: 10.0,
            max_rpm: 50000,
            coolant_pressure: 20,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurningTool {
    pub uuid: String,
    pub category: TurningToolCategory,
    pub lead_angle: f32, // DEGREE
    pub handedness: Handedness,
    pub cutting_edge_angle: f32, // DEGREE
    pub insert_type: String,
    pub maximum_ramping_angle: f32,
    pub minimum_bore_diameter: f32,
    pub workpiece_side_body_angle: f32, // DEGREE
    pub cutting_depth_maximum: f32,
    pub machine_side_body_angle: f32, // DEGREE
    pub minimum_overhang: f32,
    pub maximum_overhang: f32,
    pub usable_length: f32,
    pub body_length: f32,
    pub body_diameter: f32,
    pub functional_diameter: f32,
    pub peripheral_effective_cutting: f32,
    pub connection_diameter: f32,
    pub maximum_rpm: u32,
    pub tool_weight: f32,
}

impl Default for TurningTool {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            category: TurningToolCategory::Empty,
            lead_angle: 0.0,
            handedness: Handedness::default(),
            cutting_edge_angle: 0.0,
            insert_type: "".to_string(),
            maximum_ramping_angle: 90.0,
            minimum_bore_diameter: 0.1,
            workpiece_side_body_angle: 0.0,
            cutting_depth_maximum: 1.0,
            machine_side_body_angle: 0.0,
            minimum_overhang: 0.0,
            maximum_overhang: 10.0,
            usable_length: 5.0,
            body_length: 10.0,
            body_diameter: 5.0,
            functional_diameter: 10.0,
            peripheral_effective_cutting: 1.0,
            connection_diameter: 5.0,
            maximum_rpm: 50000,
            tool_weight: 20.0, // GRAMS
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum Handedness {
    #[default]
    Neutral,
    Left,
    Right,
}

impl fmt::Display for Handedness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Handedness::Neutral => write!(f, "Neutral"),
            Handedness::Left => write!(f, "Left"),
            Handedness::Right => write!(f, "Right"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Sequence, Default)]
pub enum RotatingToolCategory {
    #[default]
    Empty,
    BallNoseMill,
    BoringTool,
    ChamferMill,
    DoveTailCutter,
    DrillBit,
    EndMill,
    FaceMill,
    Reamer,
    SlotDrill,
    ThreadMill,
    TSlotCutter,
}
impl fmt::Display for RotatingToolCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotatingToolCategory::Empty => write!(f, "Empty"),
            RotatingToolCategory::BallNoseMill => write!(f, "Ball Nose"),
            RotatingToolCategory::BoringTool => write!(f, "Boring"),
            RotatingToolCategory::ChamferMill => write!(f, "Chamfer Mill"),
            RotatingToolCategory::DoveTailCutter => write!(f, "Dove Tail Cutter"),
            RotatingToolCategory::DrillBit => write!(f, "Drill Bit"),
            RotatingToolCategory::EndMill => write!(f, "EndMill"),
            RotatingToolCategory::FaceMill => write!(f, "Face Mill"),
            RotatingToolCategory::Reamer => write!(f, "Reamer"),
            RotatingToolCategory::SlotDrill => write!(f, "SlotDrill"),
            RotatingToolCategory::ThreadMill => write!(f, "ThreadMill"),
            RotatingToolCategory::TSlotCutter => write!(f, "T-Slot Cutter"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Sequence, Default)]
pub enum TurningToolCategory {
    #[default]
    Empty,
    InternalTurningTool,
    ExternalTurningTool,
    FacingTool,
    BoringBar,
    ThreadingTool,
    GroovingPartingTool,
    FormTool,
}

impl fmt::Display for TurningToolCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TurningToolCategory::Empty => write!(f, "Empty"),
            TurningToolCategory::InternalTurningTool => write!(f, "Internal Turning"),
            TurningToolCategory::ExternalTurningTool => write!(f, "External Turning"),
            TurningToolCategory::FacingTool => write!(f, "Facing"),
            TurningToolCategory::BoringBar => write!(f, "Boring Bar"),
            TurningToolCategory::ThreadingTool => write!(f, "Threading"),
            TurningToolCategory::GroovingPartingTool => write!(f, "Grooving/Parting"),
            TurningToolCategory::FormTool => write!(f, "Form"),
        }
    }
}
