use eframe::egui;

pub struct CncApp {
    rpm_vc: String, rpm_d: String, rpm_result: String,
    feed_n: String, feed_z: String, feed_fz: String, feed_result: String,
    active_tab: usize,
}

impl Default for CncApp {
    fn default() -> Self {
        Self {
            rpm_vc: "100".to_owned(), rpm_d: "10".to_owned(), rpm_result: String::new(),
            feed_n: "1000".to_owned(), feed_z: "4".to_owned(), feed_fz: "0.1".to_owned(), feed_result: String::new(),
            active_tab: 0,
        }
    }
}

impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka v1.2");
            ui.horizontal(|ui| {
                if ui.selectable_label(self.active_tab == 0, "Otáčky").clicked() { self.active_tab = 0; }
                if ui.selectable_label(self.active_tab == 1, "Posuv").clicked() { self.active_tab = 1; }
            });
            ui.separator();
            if self.active_tab == 0 {
                ui.label("Vc (m/min):"); ui.text_edit_singleline(&mut self.rpm_vc);
                ui.label("D (mm):"); ui.text_edit_singleline(&mut self.rpm_d);
                if ui.button("Vypočítat").clicked() {
                    let vc = self.rpm_vc.parse::<f64>().unwrap_or(0.0);
                    let d = self.rpm_d.parse::<f64>().unwrap_or(0.0);
                    if d > 0.0 { self.rpm_result = format!("{:.0} ot/min", (vc * 1000.0) / (3.14159 * d)); }
                }
                ui.label(egui::RichText::new(&self.rpm_result).size(20.0));
            } else {
                ui.label("n (ot/min):"); ui.text_edit_singleline(&mut self.feed_n);
                ui.label("z:"); ui.text_edit_singleline(&mut self.feed_z);
                ui.label("fz:"); ui.text_edit_singleline(&mut self.feed_fz);
                if ui.button("Vypočítat").clicked() {
                    let n = self.feed_n.parse::<f64>().unwrap_or(0.0);
                    let z = self.feed_z.parse::<f64>().unwrap_or(0.0);
                    let fz = self.feed_fz.parse::<f64>().unwrap_or(0.0);
                    self.feed_result = format!("{:.1} mm/min", n * z * fz);
                }
                ui.label(egui::RichText::new(&self.feed_result).size(20.0));
            }
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    eframe::run_native("CNC", options, Box::new(|_| Box::new(CncApp::default()))).unwrap();
}
