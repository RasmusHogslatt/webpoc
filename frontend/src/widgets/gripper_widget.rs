use egui::{
    Align2, Color32, Context, FontId, Frame, Painter, Pos2, Rect, Slider, Stroke, Ui, Vec2, Window,
};
use serde::{Deserialize, Serialize};

pub const SEGMENT_HEIGHT: f32 = 15.0;
pub const STROKE_HEIGHT: f32 = 20.0;
pub const OFFSET_FROM_RIGHT: f32 = 35.0;
pub const CHUCK_END: f32 = 60.0;
pub const TEXT_SEGMENT_OFFSET_SMALL: f32 = 20.0;
pub const TEXT_SEGMENT_OFFSET_LARGE: f32 = 60.0;

pub const VISUALIZATION_FRAME_WIDTH: f32 = 780.0;
pub const VISUALIZATION_FRAME_HEIGHT: f32 = 400.0;
pub const OPTION_FRAME_WIDTH: f32 = VISUALIZATION_FRAME_WIDTH / 2.0 - 20.0;
pub const OPTION_FRAME_HEIGHT: f32 = 300.0;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub margin_from_cut: f32,
    pub claw_origin_offset: f32,
    pub claw_max_grip_diameter: f32,
    pub claw_total_diameter: f32,
    // pub backdistancething
    // Machine data
    pub z_zero: f32,
    pub desired_safety_margin: f32,
    // Colors
    right_facing_stock_color: Color32,
    left_facing_stock_color: Color32,
    workpiece_color: Color32,
    cutter_color: Color32,
    gripper_color: Color32,
    bar_color: Color32,
    z_zero_color: Color32,
    offset_from_cut_color: Color32,
    pub desired_safety_margin_color: Color32,
    // Position values
    pub desired_safety_margin_end: f32,
    pub gripping_extension_end: f32,
    pub left_facing_stock_end: f32,
    pub workpiece_end: f32,
    pub right_facing_stock_end: f32,
    // Calculation results
    pub total_length_per_piece: f32,
    pub total_possible_pieces: f32,
    pub unused_material: f32,
}
//
impl Default for GripperCalculationData {
    fn default() -> Self {
        Self {
            bar_diameter: 50.0,
            bar_length: 200.0,
            workpiece_length: 100.0,
            right_facing_stock: 2.0,
            left_facing_stock: 2.0,
            cutter_width: 10.0,
            gripper_overextension: 10.0,
            gripping_point: 100.0,
            z_zero: 0.0,
            right_facing_stock_color: Color32::from_rgba_unmultiplied(60, 186, 167, 255),
            left_facing_stock_color: Color32::from_rgba_unmultiplied(60, 186, 167, 255),
            workpiece_color: Color32::from_rgba_unmultiplied(0, 66, 255, 255),
            cutter_color: Color32::from_rgba_unmultiplied(228, 19, 30, 255),
            gripper_color: Color32::from_rgba_unmultiplied(47, 39, 38, 255),
            bar_color: Color32::GRAY,
            z_zero_color: Color32::DARK_RED,
            desired_safety_margin: 2.0,
            desired_safety_margin_end: 0.0,
            gripping_extension_end: 0.0,
            left_facing_stock_end: 0.0,
            workpiece_end: 0.0,
            right_facing_stock_end: 0.0,
            total_length_per_piece: 1.0,
            desired_safety_margin_color: Color32::from_rgba_unmultiplied(156, 100, 200, 255),
            margin_from_cut: 2.0,
            offset_from_cut_color: Color32::from_rgba_unmultiplied(156, 100, 200, 255),
            total_possible_pieces: 0.0,
            unused_material: 0.0,
            claw_origin_offset: 5.0,
            claw_max_grip_diameter: 40.0,
            claw_total_diameter: 45.0,
        }
    }
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
            .resizable(false)
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
            ui.horizontal(|ui| {
                // Options frame
                Frame::none()
                    .stroke(Stroke::new(1.0, Color32::BLACK))
                    .show(ui, |ui| {
                        ui.set_width(OPTION_FRAME_WIDTH);
                        ui.set_height(OPTION_FRAME_HEIGHT);
                        ui.vertical(|ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                self.options_ui(ui);
                            });
                        });
                    });
                ui.add_space(30.0);

                Frame::none()
                    .stroke(Stroke::new(1.0, Color32::BLACK))
                    .show(ui, |ui| {
                        ui.set_width(OPTION_FRAME_WIDTH);
                        ui.set_height(OPTION_FRAME_HEIGHT);

                        self.results_ui(ui);
                    });
            });
            ui.add_space(20.0);

            // Visualization frame
            self.visualization_frame(ui);
        });
    }

    fn results_ui(&mut self, ui: &mut Ui) {
        let data = &self.gripper_calculation_data;
        egui::Grid::new("results_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Length per piece:");
                ui.label(format!("{:.2} mm", data.total_length_per_piece))
                    .on_hover_ui(|ui| {
                        egui::Grid::new("hover_results")
                            .num_columns(1)
                            .show(ui, |ui| {
                                ui.label(format!(
                                    "{:.2} mm (Safety margin)",
                                    data.desired_safety_margin
                                ));
                                ui.end_row();
                                ui.label(format!(
                                    "+ {:.2} mm (Gripper Extension)",
                                    data.gripper_overextension
                                ));
                                ui.end_row();
                                ui.label(format!(
                                    "+ {:.2} mm (Offset from cut)",
                                    data.margin_from_cut
                                ));
                                ui.end_row();
                                ui.label(format!("+ {:.2} mm (Cutter width)", data.cutter_width));
                                ui.end_row();
                                ui.label(format!(
                                    "+ {:.2} mm (Left facing stock)",
                                    data.left_facing_stock
                                ));
                                ui.end_row();
                                ui.label(format!(
                                    "+ {:.2} mm (Workpiece length)",
                                    data.workpiece_length
                                ));
                                ui.end_row();
                                ui.separator();
                                ui.end_row();
                                ui.label(format!("= {:.2} mm", data.total_length_per_piece));
                            });
                    });
                ui.end_row();
                ui.label(format!("Maximum possible nr. of pieces:"));
                ui.label(format!("{:.0}", data.total_possible_pieces.floor()));
                ui.end_row();
                ui.label("Unused material:");
                ui.label(format!("{:.2} mm", data.unused_material));
            });
    }

    fn options_ui(&mut self, ui: &mut Ui) {
        let data = &mut self.gripper_calculation_data;

        data.desired_safety_margin_end = data.desired_safety_margin;
        data.gripping_extension_end = data.desired_safety_margin_end + data.gripper_overextension;
        data.left_facing_stock_end = data.gripping_extension_end + data.left_facing_stock;
        data.workpiece_end = data.left_facing_stock_end + data.workpiece_length;
        data.right_facing_stock_end = data.workpiece_end + data.right_facing_stock;
        data.total_length_per_piece = data.desired_safety_margin
            + data.gripper_overextension
            + data.margin_from_cut
            + data.cutter_width
            + data.left_facing_stock
            + data.workpiece_length
            + data.right_facing_stock;
        data.total_possible_pieces = data.bar_length / data.total_length_per_piece;
        data.unused_material = data.total_possible_pieces.fract() * data.total_length_per_piece;
        data.gripping_point = data.z_zero
            + data.margin_from_cut
            + data.cutter_width
            + data.left_facing_stock
            + data.workpiece_length;

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
                ui.add(
                    Slider::new(&mut data.margin_from_cut, 0.0..=10.0).text("Offset from cut (mm)"),
                );
            });
        });
        ui.horizontal(|ui| {
            ui.add(
                Slider::new(&mut data.desired_safety_margin, 0.0..=10.0).text("Safety Margin (mm)"),
            );
        });

        if ui.button("Reset").clicked() {
            *self.gripper_calculation_data = GripperCalculationData::default();
        }
    }

    fn visualization_frame(&mut self, ui: &mut Ui) {
        let frame_size = Vec2::new(VISUALIZATION_FRAME_WIDTH, VISUALIZATION_FRAME_HEIGHT);

        Frame::none()
            .stroke(Stroke::new(1.0, Color32::BLACK))
            .fill(Color32::from_black_alpha(20))
            .show(ui, |ui| {
                ui.set_max_width(frame_size.x);
                ui.set_max_height(frame_size.y);

                // Create a custom frame for the visualization
                let (response, painter) = ui.allocate_painter(frame_size, egui::Sense::hover());
                let frame_rect = response.rect;

                // Paint within the frame using local coordinates
                self.paint(&painter, frame_rect);
            });
    }

    fn paint(&mut self, painter: &Painter, frame_rect: Rect) {
        let data = &self.gripper_calculation_data.clone();

        // Calculate the scaling factor to fit the detail within the frame
        let frame_width = frame_rect.width() - OFFSET_FROM_RIGHT * 2.0;
        let scale_factor = frame_width / data.total_length_per_piece;

        // Common values
        let center_y = frame_rect.center().y;
        let below_bar_y = center_y + data.bar_diameter / 2.0;
        let above_bar_y = center_y - data.bar_diameter / 2.0;

        // Draw chuck
        let chuck_start = frame_rect.left();
        let chuck_end = chuck_start + CHUCK_END;
        // Smallest diameter
        let small_chuck_height = 40.0;
        painter.rect_filled(
            Rect::from_x_y_ranges(
                chuck_start..=chuck_end,
                above_bar_y - small_chuck_height..=above_bar_y,
            ),
            3.0,
            Color32::DARK_GRAY,
        );
        painter.rect_filled(
            Rect::from_x_y_ranges(
                chuck_start..=chuck_end,
                below_bar_y..=below_bar_y + small_chuck_height,
            ),
            3.0,
            Color32::DARK_GRAY,
        );
        // Medium diameter
        let medium_chuck_height = 60.0;
        let medium_chuck_end = chuck_end - CHUCK_END / 4.0;
        painter.rect_filled(
            Rect::from_x_y_ranges(
                chuck_start..=medium_chuck_end,
                above_bar_y - medium_chuck_height..=above_bar_y,
            ),
            3.0,
            Color32::DARK_GRAY,
        );
        painter.rect_filled(
            Rect::from_x_y_ranges(
                chuck_start..=medium_chuck_end,
                below_bar_y..=below_bar_y + medium_chuck_height,
            ),
            3.0,
            Color32::DARK_GRAY,
        );
        // Largest diameter
        let large_chuck_height = 80.0;
        let large_chuck_end = chuck_end - CHUCK_END / 2.0;
        painter.rect_filled(
            Rect::from_x_y_ranges(
                chuck_start..=large_chuck_end,
                above_bar_y - large_chuck_height..=above_bar_y,
            ),
            3.0,
            Color32::DARK_GRAY,
        );
        painter.rect_filled(
            Rect::from_x_y_ranges(
                chuck_start..=large_chuck_end,
                below_bar_y..=below_bar_y + large_chuck_height,
            ),
            3.0,
            Color32::DARK_GRAY,
        );

        // Draw the bar
        let bar_start = chuck_end - (data.bar_length - data.total_length_per_piece) * scale_factor;
        let bar_end = chuck_end + data.total_length_per_piece * scale_factor;
        // let bar_end = self.to_pixel(data.bar_length, scale_factor, bar_start);
        painter.rect_filled(
            Rect::from_x_y_ranges(bar_start..=bar_end, above_bar_y..=below_bar_y),
            0.0,
            data.bar_color,
        );
        // Draw safety margin
        let safety_margin_start = chuck_end;
        let safety_margin_end = safety_margin_start + data.desired_safety_margin_end * scale_factor;
        self.draw_section(
            &painter,
            safety_margin_start,
            safety_margin_end,
            SEGMENT_HEIGHT,
            TEXT_SEGMENT_OFFSET_LARGE,
            center_y,
            data.desired_safety_margin_color,
            "Safety Margin",
            false,
            data.desired_safety_margin,
        );

        // GRIPPER START
        // Draw gripper overextension
        let gripper_extension_start = safety_margin_end;
        let gripper_extension_end =
            gripper_extension_start + data.gripper_overextension * scale_factor;
        // self.draw_section(
        //     &painter,
        //     gripper_extension_start,
        //     gripper_extension_end,
        //     SEGMENT_HEIGHT,
        //     TEXT_SEGMENT_OFFSET_SMALL,
        //     center_y,
        //     data.gripper_color,
        //     "Gripper Extension",
        //     true,
        //     data.gripper_overextension,
        // );
        // // Arrow for gripping point
        // let arrow_length = 85.0;
        // painter.arrow(
        //     Pos2::new(gripper_extension_end, above_bar_y - arrow_length),
        //     Vec2::new(0.0, arrow_length),
        //     Stroke::new(2.0, data.gripper_color),
        // );
        // let gripping_point_text = format!("Gripping Point: {:.2} mm", data.gripping_point);
        // self.draw_text(
        //     &painter,
        //     &gripping_point_text,
        //     Pos2::new(gripper_extension_end, above_bar_y - arrow_length - 20.0),
        //     data.gripper_color,
        // );
        self.draw_claw(
            &painter,
            gripper_extension_end,
            1.0,
            self.gripper_calculation_data.bar_diameter,
            center_y,
            data.gripper_color,
            "Custom claw",
            data.gripper_overextension,
            40.0,
            10.0,
        );

        // GRIPPER END

        // Draw offset from cut
        let offset_from_cut_start = gripper_extension_end;
        let offset_from_cut_end = offset_from_cut_start + data.margin_from_cut * scale_factor;
        self.draw_section(
            &painter,
            offset_from_cut_start,
            offset_from_cut_end,
            SEGMENT_HEIGHT,
            TEXT_SEGMENT_OFFSET_LARGE,
            center_y,
            data.offset_from_cut_color,
            "Offset from Cut",
            true,
            data.margin_from_cut,
        );

        // Draw cutter
        let cutter_start = offset_from_cut_end;
        let cutter_end = cutter_start + data.cutter_width * scale_factor;
        self.draw_section(
            &painter,
            cutter_start,
            cutter_end,
            SEGMENT_HEIGHT,
            TEXT_SEGMENT_OFFSET_SMALL,
            center_y,
            data.cutter_color,
            "Cutter",
            false,
            data.cutter_width,
        );

        // Draw left facing stock
        let left_facing_stock_start = cutter_end;
        let left_facing_stock_end = left_facing_stock_start + data.left_facing_stock * scale_factor;
        self.draw_section(
            &painter,
            left_facing_stock_start,
            left_facing_stock_end,
            SEGMENT_HEIGHT,
            TEXT_SEGMENT_OFFSET_LARGE,
            center_y,
            data.left_facing_stock_color,
            "Left Facing Stock",
            false,
            data.left_facing_stock,
        );

        // Draw workpiece
        let workpiece_start = left_facing_stock_end;
        let workpiece_end = workpiece_start + data.workpiece_length * scale_factor;
        self.draw_section(
            &painter,
            workpiece_start,
            workpiece_end,
            SEGMENT_HEIGHT,
            TEXT_SEGMENT_OFFSET_SMALL,
            center_y,
            data.workpiece_color,
            "Workpiece",
            false,
            data.workpiece_length,
        );

        // Draw right facing stock
        let right_facing_stock_start = workpiece_end;
        let right_facing_stock_end =
            right_facing_stock_start + data.right_facing_stock * scale_factor;
        self.draw_section(
            &painter,
            right_facing_stock_start,
            right_facing_stock_end,
            SEGMENT_HEIGHT,
            TEXT_SEGMENT_OFFSET_LARGE,
            center_y,
            data.right_facing_stock_color,
            "Right Facing Stock",
            false,
            data.right_facing_stock,
        );

        // Draw Z Zero
        let z_zero_height_offset = 50.0;
        let z_zero_start = workpiece_end;
        painter.line_segment(
            [
                Pos2::new(z_zero_start, above_bar_y - z_zero_height_offset),
                Pos2::new(z_zero_start, below_bar_y + z_zero_height_offset),
            ],
            Stroke::new(1.0, data.z_zero_color),
        );
        self.draw_text(
            &painter,
            "Z Zero",
            Pos2::new(z_zero_start, above_bar_y - z_zero_height_offset - 20.0),
            data.z_zero_color,
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

    pub fn draw_claw(
        &self,
        painter: &Painter,
        end: f32,
        claw_width: f32,
        diameter: f32,
        center_y: f32,
        color: Color32,
        text: &str,
        raw_value: f32,
        arm_length: f32,
        leg_length: f32,
    ) -> f32 {
        // Draw the circle
        painter.circle_filled(Pos2::new(end, center_y), 5.0, color);

        // Calculate the target y-coordinate (bottom of the bar)
        let target_y = center_y - diameter / 2.0;

        // Calculate the angle needed for alignment
        let dx = leg_length;
        let dy = target_y - center_y;
        let required_angle = (dy / arm_length).atan();

        // Calculate the rotated endpoints of the L shape
        let (sin, cos) = required_angle.sin_cos();
        let arm_end_x = end + arm_length * sin;
        let arm_end_y = center_y - arm_length * cos;

        // Calculate the leg endpoint
        let leg_end_x = arm_end_x - leg_length * cos;
        let leg_end_y = arm_end_y - leg_length * sin;

        // Draw the vertical part of the L (arm)
        painter.line_segment(
            [Pos2::new(end, center_y), Pos2::new(arm_end_x, arm_end_y)],
            Stroke::new(claw_width, color),
        );

        // Draw the horizontal part of the L (leg)
        painter.line_segment(
            [
                Pos2::new(arm_end_x, arm_end_y),
                Pos2::new(leg_end_x, leg_end_y),
            ],
            Stroke::new(claw_width, color),
        );

        // Draw the text
        let text_pos = Pos2::new(leg_end_x + 10.0, leg_end_y);
        painter.text(
            text_pos,
            Align2::LEFT_CENTER,
            format!("{} ({:.2} mm)", text, raw_value),
            FontId::default(),
            color,
        );

        // Return the calculated angle in radians
        required_angle
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
                Rect::from_x_y_ranges(start..=end, bar_edge - segment_height..=bar_edge),
                0.0,
                color,
            );
            // Center of segment to text
            painter.line_segment(
                [
                    Pos2::new(start + (end - start) / 2.0, bar_edge - segment_height),
                    Pos2::new(
                        start + (end - start) / 2.0,
                        bar_edge - segment_height - segment_offset + 15.0,
                    ),
                ],
                Stroke::new(1.0, color),
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset - segment_height),
                Align2::CENTER_TOP,
                text.to_owned(),
                FontId::default(),
                color,
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset - segment_height - 20.0),
                Align2::CENTER_TOP,
                format!("{:.4} mm", raw_value),
                FontId::default(),
                color,
            );
        } else {
            let bar_edge = center_y + (self.gripper_calculation_data.bar_diameter / 2.0);
            let bar_edge_with_offset = bar_edge + segment_offset;
            painter.rect_filled(
                Rect::from_x_y_ranges(start..=end, bar_edge..=bar_edge + segment_height),
                0.0,
                color,
            );
            // Center of segment to text
            painter.line_segment(
                [
                    Pos2::new(start + (end - start) / 2.0, bar_edge + segment_height),
                    Pos2::new(
                        start + (end - start) / 2.0,
                        bar_edge + segment_height + segment_offset - 15.0,
                    ),
                ],
                Stroke::new(1.0, color),
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset + segment_height),
                Align2::CENTER_BOTTOM,
                text.to_owned(),
                FontId::default(),
                color,
            );
            painter.text(
                Pos2::new(center_range, bar_edge_with_offset + segment_height + 20.0),
                Align2::CENTER_BOTTOM,
                format!("{:.2} mm", raw_value),
                FontId::default(),
                color,
            );
        }
    }
}
