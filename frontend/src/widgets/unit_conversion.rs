use crate::{app_states::WidgetState, singletons::Singletons};
use egui::{Context, Separator, Window};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LengthUnits {
    m: f32,
    cm: f32,
    mm: f32,
    micrometer: f32,
    inch: f32,
    ft: f32,
    yd: f32,
}

impl LengthUnits {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("length_units")
            .num_columns(2)
            .show(ui, |ui| {
                if ui.add(egui::DragValue::new(&mut self.micrometer)).changed() {
                    self.mm = micrometer_to_mm(self.micrometer);
                    self.cm = micrometer_to_cm(self.micrometer);
                    self.m = micrometer_to_m(self.micrometer);
                    self.inch = micrometer_to_inch(self.micrometer);
                    self.ft = micrometer_to_ft(self.micrometer);
                    self.yd = micrometer_to_yd(self.micrometer);
                }
                ui.label("µm");
                ui.end_row();
                if ui.add(egui::DragValue::new(&mut self.mm)).changed() {
                    self.micrometer = mm_to_micrometer(self.mm);
                    self.cm = mm_to_cm(self.mm);
                    self.m = mm_to_m(self.mm);
                    self.inch = mm_to_inch(self.mm);
                    self.ft = mm_to_ft(self.mm);
                    self.yd = m_to_yd(self.mm);
                }
                ui.label("mm");
                ui.end_row();
                if ui.add(egui::DragValue::new(&mut self.cm)).changed() {
                    self.micrometer = cm_to_micrometer(self.cm);
                    self.mm = cm_to_mm(self.cm);
                    self.m = cm_to_m(self.mm);
                    self.inch = cm_to_inch(self.cm);
                    self.ft = cm_to_ft(self.cm);
                    self.yd = cm_to_yd(self.cm);
                }
                ui.label("cm");
                ui.end_row();
                if ui.add(egui::DragValue::new(&mut self.m)).changed() {
                    self.micrometer = m_to_micrometer(self.m);
                    self.mm = m_to_mm(self.m);
                    self.cm = m_to_cm(self.m);
                    self.inch = m_to_inch(self.m);
                    self.ft = m_to_ft(self.m);
                    self.yd = m_to_yd(self.m);
                }
                ui.label("m");
                ui.end_row();
                if ui.add(egui::DragValue::new(&mut self.inch)).changed() {
                    self.micrometer = inch_to_micrometer(self.inch);
                    self.mm = inch_to_mm(self.inch);
                    self.cm = inch_to_cm(self.inch);
                    self.m = inch_to_m(self.inch);
                    self.ft = inch_to_ft(self.inch);
                    self.yd = inch_to_yd(self.inch);
                }
                ui.label("in");
                ui.end_row();
                if ui.add(egui::DragValue::new(&mut self.ft)).changed() {
                    self.micrometer = ft_to_micrometer(self.ft);
                    self.mm = ft_to_mm(self.ft);
                    self.cm = ft_to_cm(self.ft);
                    self.m = ft_to_m(self.ft);
                    self.inch = ft_to_inch(self.ft);
                    self.yd = ft_to_yd(self.ft);
                }
                ui.label("ft");
                ui.end_row();
                if ui.add(egui::DragValue::new(&mut self.yd)).changed() {
                    self.micrometer = yd_to_micrometer(self.yd);
                    self.mm = yd_to_mm(self.yd);
                    self.cm = yd_to_cm(self.yd);
                    self.m = yd_to_m(self.yd);
                    self.inch = yd_to_inch(self.yd);
                    self.ft = yd_to_ft(self.yd);
                }
                ui.label("yd");
                ui.end_row();
            });
    }
}

impl AreaUnits {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("area_units").num_columns(2).show(ui, |ui| {
            if ui
                .add(egui::DragValue::new(&mut self.micrometer2))
                .changed()
            {
                self.mm2 = micrometer2_to_mm2(self.micrometer2);
                self.cm2 = micrometer2_to_cm2(self.micrometer2);
                self.m2 = micrometer2_to_m2(self.micrometer2);
                self.inch2 = micrometer2_to_inch2(self.micrometer2);
                self.ft2 = micrometer2_to_ft2(self.micrometer2);
                self.yd2 = micrometer2_to_yd2(self.micrometer2);
            }
            ui.label("µm²");
            ui.end_row();

            if ui.add(egui::DragValue::new(&mut self.mm2)).changed() {
                self.micrometer2 = mm2_to_micrometer2(self.mm2);
                self.cm2 = mm2_to_cm2(self.mm2);
                self.m2 = mm2_to_m2(self.mm2);
                self.inch2 = mm2_to_inch2(self.mm2);
                self.ft2 = mm2_to_ft2(self.mm2);
                self.yd2 = mm2_to_yd2(self.mm2);
            }
            ui.label("mm²");
            ui.end_row();

            if ui.add(egui::DragValue::new(&mut self.cm2)).changed() {
                self.micrometer2 = cm2_to_micrometer2(self.cm2);
                self.mm2 = cm2_to_mm2(self.cm2);
                self.m2 = cm2_to_m2(self.cm2);
                self.inch2 = cm2_to_inch2(self.cm2);
                self.ft2 = cm2_to_ft2(self.cm2);
                self.yd2 = cm2_to_yd2(self.cm2);
            }
            ui.label("cm²");
            ui.end_row();

            if ui.add(egui::DragValue::new(&mut self.m2)).changed() {
                self.micrometer2 = m2_to_micrometer2(self.m2);
                self.mm2 = m2_to_mm2(self.m2);
                self.cm2 = m2_to_cm2(self.m2);
                self.inch2 = m2_to_inch2(self.m2);
                self.ft2 = m2_to_ft2(self.m2);
                self.yd2 = m2_to_yd2(self.m2);
            }
            ui.label("m²");
            ui.end_row();

            if ui.add(egui::DragValue::new(&mut self.inch2)).changed() {
                self.micrometer2 = inch2_to_micrometer2(self.inch2);
                self.mm2 = inch2_to_mm2(self.inch2);
                self.cm2 = inch2_to_cm2(self.inch2);
                self.m2 = inch2_to_m2(self.inch2);
                self.ft2 = inch2_to_ft2(self.inch2);
                self.yd2 = inch2_to_yd2(self.inch2);
            }
            ui.label("in²");
            ui.end_row();

            if ui.add(egui::DragValue::new(&mut self.ft2)).changed() {
                self.micrometer2 = ft2_to_micrometer2(self.ft2);
                self.mm2 = ft2_to_mm2(self.ft2);
                self.cm2 = ft2_to_cm2(self.ft2);
                self.m2 = ft2_to_m2(self.ft2);
                self.inch2 = ft2_to_inch2(self.ft2);
                self.yd2 = ft2_to_yd2(self.ft2);
            }
            ui.label("ft²");
            ui.end_row();

            if ui.add(egui::DragValue::new(&mut self.yd2)).changed() {
                self.micrometer2 = yd2_to_micrometer2(self.yd2);
                self.mm2 = yd2_to_mm2(self.yd2);
                self.cm2 = yd2_to_cm2(self.yd2);
                self.m2 = yd2_to_m2(self.yd2);
                self.inch2 = yd2_to_inch2(self.yd2);
                self.ft2 = yd2_to_ft2(self.yd2);
            }
            ui.label("yd²");
            ui.end_row();
        });
    }
}

impl VolumeUnits {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("volume_units")
            .num_columns(2)
            .show(ui, |ui| {
                if ui
                    .add(egui::DragValue::new(&mut self.micrometer3))
                    .changed()
                {
                    self.mm3 = micrometer3_to_mm3(self.micrometer3);
                    self.cm3 = micrometer3_to_cm3(self.micrometer3);
                    self.m3 = micrometer3_to_m3(self.micrometer3);
                    self.inch3 = micrometer3_to_inch3(self.micrometer3);
                    self.ft3 = micrometer3_to_ft3(self.micrometer3);
                    self.yd3 = micrometer3_to_yd3(self.micrometer3);
                }
                ui.label("µm³");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.mm3)).changed() {
                    self.micrometer3 = mm3_to_micrometer3(self.mm3);
                    self.cm3 = mm3_to_cm3(self.mm3);
                    self.m3 = mm3_to_m3(self.mm3);
                    self.inch3 = mm3_to_inch3(self.mm3);
                    self.ft3 = mm3_to_ft3(self.mm3);
                    self.yd3 = mm3_to_yd3(self.mm3);
                }
                ui.label("mm³");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.cm3)).changed() {
                    self.micrometer3 = cm3_to_micrometer3(self.cm3);
                    self.mm3 = cm3_to_mm3(self.cm3);
                    self.m3 = cm3_to_m3(self.cm3);
                    self.inch3 = cm3_to_inch3(self.cm3);
                    self.ft3 = cm3_to_ft3(self.cm3);
                    self.yd3 = cm3_to_yd3(self.cm3);
                }
                ui.label("cm³");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.m3)).changed() {
                    self.micrometer3 = m3_to_micrometer3(self.m3);
                    self.mm3 = m3_to_mm3(self.m3);
                    self.cm3 = m3_to_cm3(self.m3);
                    self.inch3 = m3_to_inch3(self.m3);
                    self.ft3 = m3_to_ft3(self.m3);
                    self.yd3 = m3_to_yd3(self.m3);
                }
                ui.label("m³");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.inch3)).changed() {
                    self.micrometer3 = inch3_to_micrometer3(self.inch3);
                    self.mm3 = inch3_to_mm3(self.inch3);
                    self.cm3 = inch3_to_cm3(self.inch3);
                    self.m3 = inch3_to_m3(self.inch3);
                    self.ft3 = inch3_to_ft3(self.inch3);
                    self.yd3 = inch3_to_yd3(self.inch3);
                }
                ui.label("in³");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.ft3)).changed() {
                    self.micrometer3 = ft3_to_micrometer3(self.ft3);
                    self.mm3 = ft3_to_mm3(self.ft3);
                    self.cm3 = ft3_to_cm3(self.ft3);
                    self.m3 = ft3_to_m3(self.ft3);
                    self.inch3 = ft3_to_inch3(self.ft3);
                    self.yd3 = ft3_to_yd3(self.ft3);
                }
                ui.label("ft³");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.yd3)).changed() {
                    self.micrometer3 = yd3_to_micrometer3(self.yd3);
                    self.mm3 = yd3_to_mm3(self.yd3);
                    self.cm3 = yd3_to_cm3(self.yd3);
                    self.m3 = yd3_to_m3(self.yd3);
                    self.inch3 = yd3_to_inch3(self.yd3);
                    self.ft3 = yd3_to_ft3(self.yd3);
                }
                ui.label("yd³");
                ui.end_row();
            });
    }
}

impl WeightUnits {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("weight_units")
            .num_columns(2)
            .show(ui, |ui| {
                if ui.add(egui::DragValue::new(&mut self.mg)).changed() {
                    self.g = mg_to_g(self.mg);
                    self.hg = mg_to_hg(self.mg);
                    self.kg = mg_to_kg(self.mg);
                    self.dr = mg_to_dr(self.mg);
                    self.oz = mg_to_oz(self.mg);
                    self.lb = mg_to_lb(self.mg);
                }
                ui.label("mg");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.g)).changed() {
                    self.mg = g_to_mg(self.g);
                    self.hg = g_to_hg(self.g);
                    self.kg = g_to_kg(self.g);
                    self.dr = g_to_dr(self.g);
                    self.oz = g_to_oz(self.g);
                    self.lb = g_to_lb(self.g);
                }
                ui.label("g");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.hg)).changed() {
                    self.mg = hg_to_mg(self.hg);
                    self.g = hg_to_g(self.hg);
                    self.kg = hg_to_kg(self.hg);
                    self.dr = hg_to_dr(self.hg);
                    self.oz = hg_to_oz(self.hg);
                    self.lb = hg_to_lb(self.hg);
                }
                ui.label("hg");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.kg)).changed() {
                    self.mg = kg_to_mg(self.kg);
                    self.g = kg_to_g(self.kg);
                    self.hg = kg_to_hg(self.kg);
                    self.dr = kg_to_dr(self.kg);
                    self.oz = kg_to_oz(self.kg);
                    self.lb = kg_to_lb(self.kg);
                }
                ui.label("kg");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.dr)).changed() {
                    self.mg = dr_to_mg(self.dr);
                    self.g = dr_to_g(self.dr);
                    self.hg = dr_to_hg(self.dr);
                    self.kg = dr_to_kg(self.dr);
                    self.oz = dr_to_oz(self.dr);
                    self.lb = dr_to_lb(self.dr);
                }
                ui.label("dr");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.oz)).changed() {
                    self.mg = oz_to_mg(self.oz);
                    self.g = oz_to_g(self.oz);
                    self.hg = oz_to_hg(self.oz);
                    self.kg = oz_to_kg(self.oz);
                    self.dr = oz_to_dr(self.oz);
                    self.lb = oz_to_lb(self.oz);
                }
                ui.label("oz");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.lb)).changed() {
                    self.mg = lb_to_mg(self.lb);
                    self.g = lb_to_g(self.lb);
                    self.hg = lb_to_hg(self.lb);
                    self.kg = lb_to_kg(self.lb);
                    self.dr = lb_to_dr(self.lb);
                    self.oz = lb_to_oz(self.lb);
                }
                ui.label("lb");
                ui.end_row();
            });
    }
}

impl TemperatureUnits {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("temperature_units")
            .num_columns(2)
            .show(ui, |ui| {
                if ui.add(egui::DragValue::new(&mut self.kelvin)).changed() {
                    self.degree = Self::kelvin_to_celsius(self.kelvin);
                    self.fahrenheit = Self::kelvin_to_fahrenheit(self.kelvin);
                }
                ui.label("K");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.degree)).changed() {
                    self.kelvin = Self::celsius_to_kelvin(self.degree);
                    self.fahrenheit = Self::celsius_to_fahrenheit(self.degree);
                }
                ui.label("°C");
                ui.end_row();

                if ui.add(egui::DragValue::new(&mut self.fahrenheit)).changed() {
                    self.kelvin = Self::fahrenheit_to_kelvin(self.fahrenheit);
                    self.degree = Self::fahrenheit_to_celsius(self.fahrenheit);
                }
                ui.label("°F");
                ui.end_row();
            });
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AreaUnits {
    m2: f32,
    cm2: f32,
    mm2: f32,
    micrometer2: f32,
    inch2: f32,
    ft2: f32,
    yd2: f32,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]

pub struct VolumeUnits {
    m3: f32,
    cm3: f32,
    mm3: f32,
    micrometer3: f32,
    inch3: f32,
    ft3: f32,
    yd3: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WeightUnits {
    mg: f32,
    g: f32,
    hg: f32,
    kg: f32,
    dr: f32,
    oz: f32,
    lb: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TemperatureUnits {
    pub kelvin: f32,
    pub degree: f32,
    pub fahrenheit: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConversionData {
    pub length: LengthUnits,
    pub area: AreaUnits,
    pub volume: VolumeUnits,
    pub weight: WeightUnits,
    pub temperature: TemperatureUnits,
}

impl ConversionData {
    // Create one for each and use grid to place individual parts nicely
    // Also move metric and and imperial length to one struct so all are updated at the same time
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("conversion_grid")
            .min_col_width(100.0)
            .max_col_width(200.0)
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                self.length.ui(ui);
                ui.add(Separator::default().vertical());
                self.area.ui(ui);
                ui.end_row();
                self.volume.ui(ui);
                ui.add(Separator::default().vertical());
                self.weight.ui(ui);
                ui.end_row();
                self.temperature.ui(ui);
            });
    }
}

pub struct UnitConversionWindow<'a> {
    singletons: &'a mut Singletons,
    widget_state: &'a mut WidgetState,
}

impl<'a> UnitConversionWindow<'a> {
    pub fn new(singletons: &'a mut Singletons, widget_state: &'a mut WidgetState) -> Self {
        Self {
            singletons,
            widget_state,
        }
    }

    pub fn show(&mut self, ctx: &Context, open: &mut bool) {
        let mut should_close = false;

        Window::new("Unit Conversion")
            .resizable(false)
            .open(open)
            .show(ctx, |ui| {
                self.singletons.conversion_data.ui(ui);

                if ui.button("Cancel").clicked() {
                    *self.widget_state = WidgetState::Default;
                    should_close = true;
                }
            });

        if should_close {
            *open = false;
        }
    }
}

pub fn m_to_cm(m: f32) -> f32 {
    m * 100.0
}
pub fn m_to_mm(m: f32) -> f32 {
    m * 1000.0
}
pub fn m_to_micrometer(m: f32) -> f32 {
    m * 1_000_000.0
}
pub fn m_to_inch(m: f32) -> f32 {
    m * 39.3701
}
pub fn m_to_ft(m: f32) -> f32 {
    m * 3.28084
}
pub fn m_to_yd(m: f32) -> f32 {
    m * 1.09361
}

// Conversion functions from centimeters
pub fn cm_to_m(cm: f32) -> f32 {
    cm / 100.0
}
pub fn cm_to_mm(cm: f32) -> f32 {
    cm * 10.0
}
pub fn cm_to_micrometer(cm: f32) -> f32 {
    cm * 10_000.0
}
pub fn cm_to_inch(cm: f32) -> f32 {
    cm / 2.54
}
pub fn cm_to_ft(cm: f32) -> f32 {
    cm / 30.48
}
pub fn cm_to_yd(cm: f32) -> f32 {
    cm / 91.44
}

// Conversion functions from millimeters
pub fn mm_to_m(mm: f32) -> f32 {
    mm / 1000.0
}
pub fn mm_to_cm(mm: f32) -> f32 {
    mm / 10.0
}
pub fn mm_to_micrometer(mm: f32) -> f32 {
    mm * 1000.0
}
pub fn mm_to_inch(mm: f32) -> f32 {
    mm / 25.4
}
pub fn mm_to_ft(mm: f32) -> f32 {
    mm / 304.8
}
pub fn mm_to_yd(mm: f32) -> f32 {
    mm / 914.4
}

// Conversion functions from micrometers
pub fn micrometer_to_m(micrometer: f32) -> f32 {
    micrometer / 1_000_000.0
}
pub fn micrometer_to_cm(micrometer: f32) -> f32 {
    micrometer / 10_000.0
}
pub fn micrometer_to_mm(micrometer: f32) -> f32 {
    micrometer / 1000.0
}
pub fn micrometer_to_inch(micrometer: f32) -> f32 {
    micrometer / 25400.0
}
pub fn micrometer_to_ft(micrometer: f32) -> f32 {
    micrometer / 304800.0
}
pub fn micrometer_to_yd(micrometer: f32) -> f32 {
    micrometer / 914400.0
}

// Conversion functions from inches
pub fn inch_to_m(inch: f32) -> f32 {
    inch * 0.0254
}
pub fn inch_to_cm(inch: f32) -> f32 {
    inch * 2.54
}
pub fn inch_to_mm(inch: f32) -> f32 {
    inch * 25.4
}
pub fn inch_to_micrometer(inch: f32) -> f32 {
    inch * 25400.0
}
pub fn inch_to_ft(inch: f32) -> f32 {
    inch / 12.0
}
pub fn inch_to_yd(inch: f32) -> f32 {
    inch / 36.0
}

// Conversion functions from feet
pub fn ft_to_m(ft: f32) -> f32 {
    ft * 0.3048
}
pub fn ft_to_cm(ft: f32) -> f32 {
    ft * 30.48
}
pub fn ft_to_mm(ft: f32) -> f32 {
    ft * 304.8
}
pub fn ft_to_micrometer(ft: f32) -> f32 {
    ft * 304800.0
}
pub fn ft_to_inch(ft: f32) -> f32 {
    ft * 12.0
}
pub fn ft_to_yd(ft: f32) -> f32 {
    ft / 3.0
}

// Conversion functions from yards
pub fn yd_to_m(yd: f32) -> f32 {
    yd * 0.9144
}
pub fn yd_to_cm(yd: f32) -> f32 {
    yd * 91.44
}
pub fn yd_to_mm(yd: f32) -> f32 {
    yd * 914.4
}
pub fn yd_to_micrometer(yd: f32) -> f32 {
    yd * 914400.0
}
pub fn yd_to_inch(yd: f32) -> f32 {
    yd * 36.0
}
pub fn yd_to_ft(yd: f32) -> f32 {
    yd * 3.0
}

// AREA
// Conversions from square meters (m2)
pub fn m2_to_cm2(m2: f32) -> f32 {
    m2 * 10000.0
}
pub fn m2_to_mm2(m2: f32) -> f32 {
    m2 * 1_000_000.0
}
pub fn m2_to_micrometer2(m2: f32) -> f32 {
    m2 * 1_000_000_000_000.0
}
pub fn m2_to_inch2(m2: f32) -> f32 {
    m2 * 1550.0031
}
pub fn m2_to_ft2(m2: f32) -> f32 {
    m2 * 10.76391
}
pub fn m2_to_yd2(m2: f32) -> f32 {
    m2 * 1.19599
}

// Conversions from square centimeters (cm2)
pub fn cm2_to_m2(cm2: f32) -> f32 {
    cm2 / 10000.0
}
pub fn cm2_to_mm2(cm2: f32) -> f32 {
    cm2 * 100.0
}
pub fn cm2_to_micrometer2(cm2: f32) -> f32 {
    cm2 * 100_000_000.0
}
pub fn cm2_to_inch2(cm2: f32) -> f32 {
    cm2 / 6.4516
}
pub fn cm2_to_ft2(cm2: f32) -> f32 {
    cm2 / 929.0304
}
pub fn cm2_to_yd2(cm2: f32) -> f32 {
    cm2 / 8361.2736
}

// Conversions from square millimeters (mm2)
pub fn mm2_to_m2(mm2: f32) -> f32 {
    mm2 / 1_000_000.0
}
pub fn mm2_to_cm2(mm2: f32) -> f32 {
    mm2 / 100.0
}
pub fn mm2_to_micrometer2(mm2: f32) -> f32 {
    mm2 * 1_000_000.0
}
pub fn mm2_to_inch2(mm2: f32) -> f32 {
    mm2 / 645.16
}
pub fn mm2_to_ft2(mm2: f32) -> f32 {
    mm2 / 92903.04
}
pub fn mm2_to_yd2(mm2: f32) -> f32 {
    mm2 / 836127.36
}

// Conversions from square micrometers (micrometer2)
pub fn micrometer2_to_m2(micrometer2: f32) -> f32 {
    micrometer2 / 1_000_000_000_000.0
}
pub fn micrometer2_to_cm2(micrometer2: f32) -> f32 {
    micrometer2 / 100_000_000.0
}
pub fn micrometer2_to_mm2(micrometer2: f32) -> f32 {
    micrometer2 / 1_000_000.0
}
pub fn micrometer2_to_inch2(micrometer2: f32) -> f32 {
    micrometer2 / 645160000.0
}
pub fn micrometer2_to_ft2(micrometer2: f32) -> f32 {
    micrometer2 / 92903040000.0
}
pub fn micrometer2_to_yd2(micrometer2: f32) -> f32 {
    micrometer2 / 836127360000.0
}

// Conversions from square inches (inch2)
pub fn inch2_to_m2(inch2: f32) -> f32 {
    inch2 * 0.00064516
}
pub fn inch2_to_cm2(inch2: f32) -> f32 {
    inch2 * 6.4516
}
pub fn inch2_to_mm2(inch2: f32) -> f32 {
    inch2 * 645.16
}
pub fn inch2_to_micrometer2(inch2: f32) -> f32 {
    inch2 * 645160000.0
}
pub fn inch2_to_ft2(inch2: f32) -> f32 {
    inch2 / 144.0
}
pub fn inch2_to_yd2(inch2: f32) -> f32 {
    inch2 / 1296.0
}

// Conversions from square feet (ft2)
pub fn ft2_to_m2(ft2: f32) -> f32 {
    ft2 * 0.092903
}
pub fn ft2_to_cm2(ft2: f32) -> f32 {
    ft2 * 929.0304
}
pub fn ft2_to_mm2(ft2: f32) -> f32 {
    ft2 * 92903.04
}
pub fn ft2_to_micrometer2(ft2: f32) -> f32 {
    ft2 * 92903040000.0
}
pub fn ft2_to_inch2(ft2: f32) -> f32 {
    ft2 * 144.0
}
pub fn ft2_to_yd2(ft2: f32) -> f32 {
    ft2 / 9.0
}

// Conversions from square yards (yd2)
pub fn yd2_to_m2(yd2: f32) -> f32 {
    yd2 * 0.836127
}
pub fn yd2_to_cm2(yd2: f32) -> f32 {
    yd2 * 8361.2736
}
pub fn yd2_to_mm2(yd2: f32) -> f32 {
    yd2 * 836127.36
}
pub fn yd2_to_micrometer2(yd2: f32) -> f32 {
    yd2 * 836127360000.0
}
pub fn yd2_to_inch2(yd2: f32) -> f32 {
    yd2 * 1296.0
}
pub fn yd2_to_ft2(yd2: f32) -> f32 {
    yd2 * 9.0
}

// VOLUME

// Conversions from cubic meters (m3)
pub fn m3_to_cm3(m3: f32) -> f32 {
    m3 * 1_000_000.0
}
pub fn m3_to_mm3(m3: f32) -> f32 {
    m3 * 1_000_000_000.0
}
pub fn m3_to_micrometer3(m3: f32) -> f32 {
    m3 * 1_000_000_000_000_000.0
}
pub fn m3_to_inch3(m3: f32) -> f32 {
    m3 * 61023.744
}
pub fn m3_to_ft3(m3: f32) -> f32 {
    m3 * 35.31467
}
pub fn m3_to_yd3(m3: f32) -> f32 {
    m3 * 1.30795
}

// Conversions from cubic centimeters (cm3)
pub fn cm3_to_m3(cm3: f32) -> f32 {
    cm3 / 1_000_000.0
}
pub fn cm3_to_mm3(cm3: f32) -> f32 {
    cm3 * 1000.0
}
pub fn cm3_to_micrometer3(cm3: f32) -> f32 {
    cm3 * 1_000_000_000.0
}
pub fn cm3_to_inch3(cm3: f32) -> f32 {
    cm3 / 16.387064
}
pub fn cm3_to_ft3(cm3: f32) -> f32 {
    cm3 / 28316.846592
}
pub fn cm3_to_yd3(cm3: f32) -> f32 {
    cm3 / 764554.857984
}

// Conversions from cubic millimeters (mm3)
pub fn mm3_to_m3(mm3: f32) -> f32 {
    mm3 / 1_000_000_000.0
}
pub fn mm3_to_cm3(mm3: f32) -> f32 {
    mm3 / 1000.0
}
pub fn mm3_to_micrometer3(mm3: f32) -> f32 {
    mm3 * 1_000_000.0
}
pub fn mm3_to_inch3(mm3: f32) -> f32 {
    mm3 / 16387.064
}
pub fn mm3_to_ft3(mm3: f32) -> f32 {
    mm3 / 28316846.592
}
pub fn mm3_to_yd3(mm3: f32) -> f32 {
    mm3 / 764554857.984
}

// Conversions from cubic micrometers (micrometer3)
pub fn micrometer3_to_m3(micrometer3: f32) -> f32 {
    micrometer3 / 1_000_000_000_000_000.0
}
pub fn micrometer3_to_cm3(micrometer3: f32) -> f32 {
    micrometer3 / 1_000_000_000.0
}
pub fn micrometer3_to_mm3(micrometer3: f32) -> f32 {
    micrometer3 / 1_000_000.0
}
pub fn micrometer3_to_inch3(micrometer3: f32) -> f32 {
    micrometer3 / 16387064000.0
}
pub fn micrometer3_to_ft3(micrometer3: f32) -> f32 {
    micrometer3 / 28316846592000.0
}
pub fn micrometer3_to_yd3(micrometer3: f32) -> f32 {
    micrometer3 / 764554857984000.0
}

// Conversions from cubic inches (inch3)
pub fn inch3_to_m3(inch3: f32) -> f32 {
    inch3 * 0.000016387064
}
pub fn inch3_to_cm3(inch3: f32) -> f32 {
    inch3 * 16.387064
}
pub fn inch3_to_mm3(inch3: f32) -> f32 {
    inch3 * 16387.064
}
pub fn inch3_to_micrometer3(inch3: f32) -> f32 {
    inch3 * 16387064000.0
}
pub fn inch3_to_ft3(inch3: f32) -> f32 {
    inch3 / 1728.0
}
pub fn inch3_to_yd3(inch3: f32) -> f32 {
    inch3 / 46656.0
}

// Conversions from cubic feet (ft3)
pub fn ft3_to_m3(ft3: f32) -> f32 {
    ft3 * 0.028316846592
}
pub fn ft3_to_cm3(ft3: f32) -> f32 {
    ft3 * 28316.846592
}
pub fn ft3_to_mm3(ft3: f32) -> f32 {
    ft3 * 28316846.592
}
pub fn ft3_to_micrometer3(ft3: f32) -> f32 {
    ft3 * 28316846592000.0
}
pub fn ft3_to_inch3(ft3: f32) -> f32 {
    ft3 * 1728.0
}
pub fn ft3_to_yd3(ft3: f32) -> f32 {
    ft3 / 27.0
}

// Conversions from cubic yards (yd3)
pub fn yd3_to_m3(yd3: f32) -> f32 {
    yd3 * 0.764554857984
}
pub fn yd3_to_cm3(yd3: f32) -> f32 {
    yd3 * 764554.857984
}
pub fn yd3_to_mm3(yd3: f32) -> f32 {
    yd3 * 764554857.984
}
pub fn yd3_to_micrometer3(yd3: f32) -> f32 {
    yd3 * 764554857984000.0
}
pub fn yd3_to_inch3(yd3: f32) -> f32 {
    yd3 * 46656.0
}
pub fn yd3_to_ft3(yd3: f32) -> f32 {
    yd3 * 27.0
}

// WEIGHT

// Conversions from milligrams (mg)
pub fn mg_to_g(mg: f32) -> f32 {
    mg / 1000.0
}
pub fn mg_to_hg(mg: f32) -> f32 {
    mg / 100000.0
}
pub fn mg_to_kg(mg: f32) -> f32 {
    mg / 1000000.0
}
pub fn mg_to_dr(mg: f32) -> f32 {
    mg / 1771.845195
}
pub fn mg_to_oz(mg: f32) -> f32 {
    mg / 28349.523125
}
pub fn mg_to_lb(mg: f32) -> f32 {
    mg / 453592.37
}

// Conversions from grams (g)
pub fn g_to_mg(g: f32) -> f32 {
    g * 1000.0
}
pub fn g_to_hg(g: f32) -> f32 {
    g / 100.0
}
pub fn g_to_kg(g: f32) -> f32 {
    g / 1000.0
}
pub fn g_to_dr(g: f32) -> f32 {
    g / 1.7718451953125
}
pub fn g_to_oz(g: f32) -> f32 {
    g / 28.349523125
}
pub fn g_to_lb(g: f32) -> f32 {
    g / 453.59237
}

// Conversions from hectograms (hg)
pub fn hg_to_mg(hg: f32) -> f32 {
    hg * 100000.0
}
pub fn hg_to_g(hg: f32) -> f32 {
    hg * 100.0
}
pub fn hg_to_kg(hg: f32) -> f32 {
    hg / 10.0
}
pub fn hg_to_dr(hg: f32) -> f32 {
    hg * 56.43833912
}
pub fn hg_to_oz(hg: f32) -> f32 {
    hg * 3.5273962
}
pub fn hg_to_lb(hg: f32) -> f32 {
    hg / 4.5359237
}

// Conversions from kilograms (kg)
pub fn kg_to_mg(kg: f32) -> f32 {
    kg * 1000000.0
}
pub fn kg_to_g(kg: f32) -> f32 {
    kg * 1000.0
}
pub fn kg_to_hg(kg: f32) -> f32 {
    kg * 10.0
}
pub fn kg_to_dr(kg: f32) -> f32 {
    kg * 564.383391
}
pub fn kg_to_oz(kg: f32) -> f32 {
    kg * 35.27396195
}
pub fn kg_to_lb(kg: f32) -> f32 {
    kg * 2.20462262
}

// Conversions from drams (dr)
pub fn dr_to_mg(dr: f32) -> f32 {
    dr * 1771.845195
}
pub fn dr_to_g(dr: f32) -> f32 {
    dr * 1.7718451953125
}
pub fn dr_to_hg(dr: f32) -> f32 {
    dr / 56.43833912
}
pub fn dr_to_kg(dr: f32) -> f32 {
    dr / 564.383391
}
pub fn dr_to_oz(dr: f32) -> f32 {
    dr / 16.0
}
pub fn dr_to_lb(dr: f32) -> f32 {
    dr / 256.0
}

// Conversions from ounces (oz)
pub fn oz_to_mg(oz: f32) -> f32 {
    oz * 28349.523125
}
pub fn oz_to_g(oz: f32) -> f32 {
    oz * 28.349523125
}
pub fn oz_to_hg(oz: f32) -> f32 {
    oz / 3.5273962
}
pub fn oz_to_kg(oz: f32) -> f32 {
    oz / 35.27396195
}
pub fn oz_to_dr(oz: f32) -> f32 {
    oz * 16.0
}
pub fn oz_to_lb(oz: f32) -> f32 {
    oz / 16.0
}

// Conversions from pounds (lb)
pub fn lb_to_mg(lb: f32) -> f32 {
    lb * 453592.37
}
pub fn lb_to_g(lb: f32) -> f32 {
    lb * 453.59237
}
pub fn lb_to_hg(lb: f32) -> f32 {
    lb * 4.5359237
}
pub fn lb_to_kg(lb: f32) -> f32 {
    lb / 2.20462262
}
pub fn lb_to_dr(lb: f32) -> f32 {
    lb * 256.0
}
pub fn lb_to_oz(lb: f32) -> f32 {
    lb * 16.0
}

// TEMPERATURE
impl TemperatureUnits {
    // Conversions from Kelvin
    pub fn kelvin_to_celsius(k: f32) -> f32 {
        k - 273.15
    }
    pub fn kelvin_to_fahrenheit(k: f32) -> f32 {
        (k - 273.15) * 9.0 / 5.0 + 32.0
    }

    // Conversions from Celsius
    pub fn celsius_to_kelvin(c: f32) -> f32 {
        c + 273.15
    }
    pub fn celsius_to_fahrenheit(c: f32) -> f32 {
        c * 9.0 / 5.0 + 32.0
    }

    // Conversions from Fahrenheit
    pub fn fahrenheit_to_kelvin(f: f32) -> f32 {
        (f - 32.0) * 5.0 / 9.0 + 273.15
    }
    pub fn fahrenheit_to_celsius(f: f32) -> f32 {
        (f - 32.0) * 5.0 / 9.0
    }
}
