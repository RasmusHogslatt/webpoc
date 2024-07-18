use egui::{
    Align2, CollapsingHeader, Color32, Context, FontId, Frame, Painter, Pos2, Rect, Slider, Stroke,
    Ui, Vec2, Window,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GripperCalculationData {
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
}

pub struct LatheBarGripperWindow<'a> {
    pub gripper_calculation_data: &'a mut GripperCalculationData,
}

impl<'a> LatheBarGripperWindow<'a> {
    pub fn new(gripper_calculation_data: &'a mut GripperCalculationData) -> Self {
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
        ui.add(
            Slider::new(&mut data.gripper_overextension, 0.0..=50.0)
                .text("Gripper Overextension (mm)"),
        );
        ui.add(
            Slider::new(&mut data.gripping_point, 0.0..=data.bar_length)
                .text("Gripping Point (mm)"),
        );
        ui.add(Slider::new(&mut data.z_zero, -100.0..=100.0).text("Z Zero (mm)"));

        ui.label(format!(
            "Current Gripping Point: {:.2} mm",
            data.gripping_point
        ));

        if ui.button("Reset").clicked() {
            *self.gripper_calculation_data = GripperCalculationData::default();
        }
    }

    fn visualization_frame(&self, ui: &mut Ui) {
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

    fn paint(&self, painter: &Painter, rect: Rect) {
        let center = rect.center();

        let data = &self.gripper_calculation_data;
        //paint rect.x_range values
        painter.text(
            rect.left_top() + Vec2::new(10.0, 10.0),
            Align2::LEFT_TOP,
            format!("Frame position: {:?} {:?}", rect.x_range(), rect.y_range()),
            FontId::default(),
            Color32::BLACK,
        );
        let start_right = rect.right() - 10.0;
        let center_y = rect.center().y;
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
        let bar_start = start_right;
        let bar_end = bar_start - data.bar_length;
        painter.rect_filled(
            Rect::from_x_y_ranges(bar_end..=bar_start, bar_top..=bar_bottom),
            0.0,
            Color32::GRAY,
        );

        // Draw right facing stock
        let right_facing_stock_start = bar_start;
        let right_facing_stock_end = right_facing_stock_start - data.right_facing_stock;
        painter.rect_filled(
            Rect::from_x_y_ranges(
                right_facing_stock_end..=right_facing_stock_start,
                bar_top..=bar_bottom,
            ),
            0.0,
            Color32::from_rgba_unmultiplied(0, 255, 255, 128),
        );

        // Draw workpiece
        let workpiece_start = right_facing_stock_end;
        let workpiece_end = workpiece_start - data.workpiece_length;
        painter.rect_filled(
            Rect::from_x_y_ranges(workpiece_end..=workpiece_start, bar_top..=bar_bottom),
            0.0,
            Color32::from_rgba_unmultiplied(0, 128, 128, 128),
        );

        // Draw left facing stock
        let left_facing_stock_start = workpiece_end;
        let left_facing_stock_end = left_facing_stock_start - data.left_facing_stock;
        painter.rect_filled(
            Rect::from_x_y_ranges(
                left_facing_stock_end..=left_facing_stock_start,
                bar_top..=bar_bottom,
            ),
            0.0,
            Color32::from_rgba_unmultiplied(0, 255, 255, 128),
        );

        // Draw the cutter
        let cutter_start = left_facing_stock_end;
        let cutter_end = cutter_start - data.cutter_width;
        painter.rect_filled(
            Rect::from_x_y_ranges(cutter_end..=cutter_start, bar_top..=bar_bottom),
            0.0,
            Color32::from_rgba_unmultiplied(255, 0, 0, 128),
        );

        // Draw the gripper
        let gripper_x = rect.left() + (data.gripping_point / data.bar_length) * rect.width();
        let gripper_width = 10.0; // Arbitrary width for visualization
        let gripper_top = center.y - data.bar_diameter / 2.0 - data.gripper_overextension;
        let gripper_bottom = center.y + data.bar_diameter / 2.0 + data.gripper_overextension;
        painter.rect_filled(
            Rect::from_x_y_ranges(
                gripper_x - gripper_width / 2.0..=gripper_x + gripper_width / 2.0,
                gripper_top..=gripper_bottom,
            ),
            0.0,
            Color32::RED,
        );

        // Draw Z zero indicator
        let z_zero_x =
            rect.left() + ((data.z_zero + data.bar_length / 2.0) / data.bar_length) * rect.width();
        painter.line_segment(
            [
                Pos2::new(z_zero_x, rect.top()),
                Pos2::new(z_zero_x, rect.bottom()),
            ],
            Stroke::new(1.0, Color32::GREEN),
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
}
