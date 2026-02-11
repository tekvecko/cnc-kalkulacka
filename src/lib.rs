use eframe::egui;

pub struct CncApp {
    rpm_vc: String,
    rpm_d: String,
    rpm_result: String,
    feed_n: String,
    feed_z: String,
    feed_fz: String,
    feed_result: String,
    active_tab: usize,
}

impl Default for CncApp {
    fn default() -> Self {
        Self {
            rpm_vc: "100".to_owned(),
            rpm_d: "10".to_owned(),
            rpm_result: String::new(),
            feed_n: "1000".to_owned(),
            feed_z: "4".to_owned(),
            feed_fz: "0.1".to_owned(),
            feed_result: String::new(),
            active_tab: 0,
        }
    }
}

impl CncApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka");
            
            ui.horizontal(|ui| {
                if ui.selectable_label(self.active_tab == 0, "Otáčky").clicked() { self.active_tab = 0; }
                if ui.selectable_label(self.active_tab == 1, "Posuv").clicked() { self.active_tab = 1; }
            });
            
            ui.separator();

            if self.active_tab == 0 {
                ui.label("Rezková rychlost Vc (m/min):");
                ui.text_edit_singleline(&mut self.rpm_vc);
                ui.label("Průměr nástroje D (mm):");
                ui.text_edit_singleline(&mut self.rpm_d);
                
                if ui.button("Vypočítat otáčky").clicked() {
                    let vc: f64 = self.rpm_vc.parse().unwrap_or(0.0);
                    let d: f64 = self.rpm_d.parse().unwrap_or(0.0);
                    if d > 0.0 {
                        let n = (vc * 1000.0) / (3.14159 * d);
                        self.rpm_result = format!("Výsledek: {:.0} ot/min", n);
                    } else {
                        self.rpm_result = "Chyba: Průměr musí být > 0".into();
                    }
                }
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&self.rpm_result).strong().size(18.0));
                
            } else {
                ui.label("Otáčky n (ot/min):");
                ui.text_edit_singleline(&mut self.feed_n);
                ui.label("Počet zubů z:");
                ui.text_edit_singleline(&mut self.feed_z);
                ui.label("Posuv na zub fz (mm):");
                ui.text_edit_singleline(&mut self.feed_fz);
                
                if ui.button("Vypočítat posuv").clicked() {
                    let n: f64 = self.feed_n.parse().unwrap_or(0.0);
                    let z: f64 = self.feed_z.parse().unwrap_or(0.0);
                    let fz: f64 = self.feed_fz.parse().unwrap_or(0.0);
                    let vf = n * z * fz;
                    self.feed_result = format!("Výsledek: {:.1} mm/min", vf);
                }
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&self.feed_result).strong().size(18.0));
            }
        });
    }
}

// Hlavní vstupní bod pro Android
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;
    
    let mut options = NativeOptions::default();
    // Vynucení Glow (OpenGL), který je na mobilech nejstabilnější
    options.renderer = eframe::Renderer::Glow;

    eframe::run_native(
        "CNC Kalkulačka",
        options,
        Box::new(|cc| Box::new(CncApp::new(cc))),
    ).unwrap();
}

