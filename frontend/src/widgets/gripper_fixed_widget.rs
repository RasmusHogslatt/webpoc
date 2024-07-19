use egui::{
    Align2, CollapsingHeader, Color32, Context, FontId, Frame, Painter, Pos2, Rect, Slider, Stroke,
    Ui, Vec2, Window,
};
use serde::{Deserialize, Serialize};

pub const SEGMENT_HEIGHT: f32 = 15.0;
pub const STROKE_HEIGHT: f32 = 20.0;
pub const OFFSET_FROM_RIGHT: f32 = 35.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GripperFixedCalculationData {
    // Material data
    pub bar_diameter: f32,
    pub bar_length: f32,
    // Workpiece data
    pub workpiece_length: f32,
    // Options
    pub right_facing_stock: f32,
    pub left_facing_stock: f32,
    // Tool data
    pub cutter_width: f32,
    // Gripper data
    pub gripper_overextension: f32,
    pub gripping_point: f32,
    // Machine data
    pub z_zero: f32,
    // Colors
    right_facing_stock_color: Color32,
    left_facing_stock_color: Color32,
    workpiece_color: Color32,
    cutter_color: Color32,
    gripper_color: Color32,
    bar_color: Color32,
    z_zero_color: Color32,
    bar_start_is_z_zero: bool,
    grip_at_end_of_cutter: bool,
}

impl Default for GripperFixedCalculationData {
    fn default() -> Self {
        Self {
            bar_diameter: 20.0,
            bar_length: 200.0,
            workpiece_length: 100.0,
            right_facing_stock: 2.0,
            left_facing_stock: 2.0,
            cutter_width: 10.0,
            gripper_overextension: 10.0,
            gripping_point: 100.0,
            z_zero: 0.0,
            right_facing_stock_color: Color32::from_rgba_unmultiplied(60, 186, 167, 200),
            left_facing_stock_color: Color32::from_rgba_unmultiplied(60, 186, 167, 200),
            workpiece_color: Color32::from_rgba_unmultiplied(0, 66, 255, 135),
            cutter_color: Color32::from_rgba_unmultiplied(228, 19, 30, 200),
            gripper_color: Color32::from_rgba_unmultiplied(47, 39, 38, 200),
            bar_color: Color32::GRAY,
            z_zero_color: Color32::YELLOW,
            bar_start_is_z_zero: true,
            grip_at_end_of_cutter: true,
        }
    }
}

pub struct LatheBarGripperFixedWindow<'a> {
    pub gripper_calculation_data: &'a mut GripperFixedCalculationData,
}

impl<'a> LatheBarGripperFixedWindow<'a> {
    pub fn new(gripper_calculation_data: &'a mut GripperFixedCalculationData) -> Self {
        Self {
            gripper_calculation_data,
        }
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let mut should_close = false;

        Window::new("Lathe Bar Gripper")
            .open(open)
            .resizable(true)
            .default_size([800.0, 800.0])
            .show(ctx, |ui| {
                self.ui_contents(ui);

                if ui.button("Close").clicked() {
                    should_close = true;
                }
            });

        if should_close {
            *open = false;
        }
    }

    fn ui_contents(&mut self, ui: &mut Ui) {
        ui.heading("Lathe Bar Gripper Calculator");

        ui.add_space(10.0);

        ui.vertical(|ui| {
            // Options frame
            Frame::none()
                .stroke(Stroke::new(1.0, Color32::GRAY))
                .show(ui, |ui| {
                    ui.set_max_width(780.0);
                    ui.set_max_height(300.0);

                    CollapsingHeader::new("Gripper Settings")
                        .default_open(true)
                        .show(ui, |ui| self.options_ui(ui));
                });

            ui.add_space(20.0);

            // Visualization frame
            self.visualization_frame(ui);
        });
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        let data = &mut self.gripper_calculation_data;

        ui.add(Slider::new(&mut data.bar_diameter, 1.0..=100.0).text("Bar Diameter (mm)"));
        ui.add(Slider::new(&mut data.bar_length, 0.0..=1000.0).text("Bar Length (mm)"));
        ui.add(Slider::new(&mut data.workpiece_length, 0.0..=500.0).text("Workpiece Length (mm)"));
        ui.add(
            Slider::new(&mut data.right_facing_stock, 0.0..=10.0).text("Right Facing Stock (mm)"),
        );
        ui.add(Slider::new(&mut data.left_facing_stock, 0.0..=10.0).text("Left Facing Stock (mm)"));
        ui.add(Slider::new(&mut data.cutter_width, 0.1..=20.0).text("Cutter Width (mm)"));

        ui.group(|ui| {
            ui.add(
                Slider::new(&mut data.gripper_overextension, 0.0..=50.0)
                    .text("Gripper Overextension (mm)"),
            );
            ui.horizontal(|ui| {
                let grip_at_end_of_cutter = data.grip_at_end_of_cutter;
                ui.add_enabled(
                    !grip_at_end_of_cutter,
                    Slider::new(&mut data.gripping_point, 0.0..=data.bar_length)
                        .text("Gripping Point (mm)"),
                );
                ui.checkbox(&mut data.grip_at_end_of_cutter, "Set to end of cutter");
            });
        });
        ui.horizontal(|ui| {
            let bar_start_is_z_zero = data.bar_start_is_z_zero;
            ui.add_enabled(
                !bar_start_is_z_zero,
                Slider::new(&mut data.z_zero, -100.0..=100.0).text("Z Zero (mm)"),
            );
            ui.checkbox(&mut data.bar_start_is_z_zero, "Set to end of bar");
        });
        if data.bar_start_is_z_zero {
            data.z_zero = 0.0;
        }

        ui.label(format!(
            "Current Gripping Point: {:.2} mm",
            data.gripping_point
        ));

        if ui.button("Reset").clicked() {
            *self.gripper_calculation_data = GripperFixedCalculationData::default();
        }
    }

    fn visualization_frame(&mut self, ui: &mut Ui) {
        Frame::none()
            .stroke(Stroke::new(1.0, Color32::GRAY))
            .show(ui, |ui| {
                ui.set_max_width(780.0);
                ui.set_max_height(400.0);
                ui.heading("Visualization");

                // Allocate space for painting
                let (response, painter) = ui
                    .allocate_painter(Vec2::new(ui.available_width(), 350.0), egui::Sense::hover());

                // Paint only within the allocated space
                self.paint(&painter, response.rect);
            });
    }

    fn paint(&mut self, painter: &Painter, rect: Rect) {
        let data = &self.gripper_calculation_data.clone();
        //paint rect.x_range values
        painter.text(
            rect.left_top() + Vec2::new(10.0, 10.0),
            Align2::LEFT_TOP,
            format!("Frame position: {:?} {:?}", rect.x_range(), rect.y_range()),
            FontId::default(),
            Color32::BLACK,
        );
        let start_right = rect.right() - OFFSET_FROM_RIGHT - data.z_zero;
        let center_y = rect.center().y;
        let below_bar_y = center_y + data.bar_diameter / 2.0;
        let above_bar_y = center_y - data.bar_diameter / 2.0;
        // Draw start right and center y
        painter.text(
            rect.left_top() + Vec2::new(10.0, 30.0),
            Align2::LEFT_TOP,
            format!("Start Right: {:?} Center Y: {:?}", start_right, center_y),
            FontId::default(),
            Color32::BLACK,
        );

        // Draw the bar
        let bar_top = center_y - data.bar_diameter / 2.0;
        let bar_bottom = center_y + data.bar_diameter / 2.0;
        let bar_start = rect.right() - OFFSET_FROM_RIGHT;
        let bar_end = bar_start - data.bar_length;
        painter.rect_filled(
            Rect::from_x_y_ranges(bar_end..=bar_start, bar_top..=bar_bottom),
            0.0,
            data.bar_color,
        );

        // Draw right facing stock
        let right_facing_stock_start = start_right;
        let right_facing_stock_end = right_facing_stock_start - data.right_facing_stock;
        self.draw_section(
            painter,
            right_facing_stock_start,
            right_facing_stock_end,
            SEGMENT_HEIGHT,
            15.0,
            center_y,
            data.right_facing_stock_color,
            "Facing Stock",
            true,
            data.right_facing_stock,
        );

        // Draw workpiece
        let workpiece_start = right_facing_stock_end;
        let workpiece_end = workpiece_start - data.workpiece_length;
        self.draw_section(
            painter,
            workpiece_start,
            workpiece_end,
            SEGMENT_HEIGHT,
            15.0,
            center_y,
            data.workpiece_color,
            "Workpiece",
            false,
            data.workpiece_length,
        );
        // Draw left facing stock
        let left_facing_stock_start = workpiece_end;
        let left_facing_stock_end = left_facing_stock_start - data.left_facing_stock;
        self.draw_section(
            painter,
            left_facing_stock_start,
            left_facing_stock_end,
            SEGMENT_HEIGHT,
            15.0,
            center_y,
            data.left_facing_stock_color,
            "Facing Stock",
            true,
            data.left_facing_stock,
        );

        // Draw the cutter
        let cutter_start = left_facing_stock_end;
        let cutter_end = cutter_start - data.cutter_width;
        self.draw_section(
            painter,
            cutter_start,
            cutter_end,
            SEGMENT_HEIGHT,
            15.0,
            center_y,
            data.cutter_color,
            "Cutter",
            false,
            data.cutter_width,
        );

        // Draw the gripper
        let mut gripper_start = bar_start - data.gripping_point;
        let mut gripper_end = gripper_start - data.gripper_overextension;
        if data.grip_at_end_of_cutter {
            gripper_start = cutter_end;
            gripper_end = gripper_start - data.gripper_overextension;
        }

        self.draw_section(
            painter,
            gripper_start,
            gripper_end,
            SEGMENT_HEIGHT,
            15.0,
            center_y,
            data.gripper_color,
            "Gripper extension",
            true,
            data.gripper_overextension,
        );
        painter.arrow(
            Pos2::new(gripper_start, center_y + 5.0 * SEGMENT_HEIGHT),
            Vec2::new(0.0, -(center_y + 5.0 * SEGMENT_HEIGHT - above_bar_y).abs()),
            Stroke::new(1.0, data.gripper_color),
        );

        let gripping_point_text = if data.grip_at_end_of_cutter {
            format!("Grip point: {:.2} mm", bar_start - gripper_start)
        } else {
            format!("Grip point: {:.2} mm", data.gripping_point)
        };

        painter.text(
            Pos2::new(gripper_start, center_y + 5.0 * SEGMENT_HEIGHT),
            Align2::CENTER_TOP,
            gripping_point_text,
            FontId::default(),
            data.gripper_color,
        );

        // Draw Z zero indicator
        let z_zero_x = bar_start - data.z_zero;
        painter.line_segment(
            [
                Pos2::new(z_zero_x, rect.top()),
                Pos2::new(z_zero_x, rect.bottom()),
            ],
            Stroke::new(1.0, data.z_zero_color),
        );

        // Draw labels
        painter.text(
            rect.left_bottom() + Vec2::new(10.0, -20.0),
            Align2::LEFT_BOTTOM,
            format!("Gripping Point: {:.2} mm", data.gripping_point),
            FontId::default(),
            Color32::WHITE,
        );
        painter.text(
            rect.right_bottom() + Vec2::new(-10.0, -20.0),
            Align2::RIGHT_BOTTOM,
            format!("Z Zero: {:.2} mm", data.z_zero),
            FontId::default(),
            Color32::GREEN,
        );
    }

    pub fn draw_text(&self, painter: &Painter, text: &str, pos: Pos2, color: Color32) {
        painter.text(
            pos,
            Align2::CENTER_CENTER,
            text.to_owned(),
            FontId::default(),
            color,
        );
    }

    pub fn draw_section(
        &self,
        painter: &Painter,
        start: f32,
        end: f32,
        segment_height: f32,
        segment_offset: f32,
        center_y: f32,
        color: Color32,
        text: &str,
        above: bool,
        raw_value: f32,
    ) {
        let center_range = (start + end) / 2.0;
        if above {
            let bar_edge = center_y - (self.gripper_calculation_data.bar_diameter / 2.0);
            let bar_edge_with_offset = bar_edge - segment_offset;
            painter.rect_filled(
                Rect::from_x_y_ranges(
                    end..=start,
                    bar_edge_with_offset - segment_height..=bar_edge_with_offset,
                ),
                0.0,
                color,
            );
            painter.line_segment(
                [
                    Pos2::new(start, bar_edge),
                    Pos2::new(start, bar_edge_with_offset - segment_height),
                ],
                Stroke::new(1.0, color),
            );
            painter.line_segment(
                [
                    Pos2::new(end, bar_edge),
                    Pos2::new(end, bar_edge_with_offset - segment_height),
                ],
                Stroke::new(1.0, color),
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset - 2.0 * segment_height),
                Align2::CENTER_TOP,
                text.to_owned(),
                FontId::default(),
                color,
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset - 3.0 * segment_height),
                Align2::CENTER_TOP,
                format!("{:.2} mm", raw_value),
                FontId::default(),
                color,
            );
        } else {
            let bar_edge = center_y + (self.gripper_calculation_data.bar_diameter / 2.0);
            let bar_edge_with_offset = bar_edge + segment_offset;
            painter.rect_filled(
                Rect::from_x_y_ranges(
                    end..=start,
                    bar_edge_with_offset..=bar_edge_with_offset + segment_height,
                ),
                0.0,
                color,
            );
            painter.line_segment(
                [
                    Pos2::new(start, bar_edge),
                    Pos2::new(start, bar_edge_with_offset + segment_height),
                ],
                Stroke::new(1.0, color),
            );
            painter.line_segment(
                [
                    Pos2::new(end, bar_edge),
                    Pos2::new(end, bar_edge_with_offset + segment_height),
                ],
                Stroke::new(1.0, color),
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset + 2.0 * segment_height),
                Align2::CENTER_BOTTOM,
                text.to_owned(),
                FontId::default(),
                color,
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset + 3.0 * segment_height),
                Align2::CENTER_BOTTOM,
                format!("{:.2} mm", raw_value),
                FontId::default(),
                color,
            );
        }
    }
}
