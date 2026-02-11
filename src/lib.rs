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
                ui.label("Vc (m/min):"); ui.text_edit_singleline(&mut self.rpm_vc);
                ui.label("D (mm):"); ui.text_edit_singleline(&mut self.rpm_d);
                if ui.button("Vypočítat").clicked() {
                    let vc: f64 = self.rpm_vc.parse().unwrap_or(0.0);
                    let d: f64 = self.rpm_d.parse().unwrap_or(0.0);
                    self.rpm_result = if d > 0.0 { format!("{:.0} ot/min", (vc * 1000.0) / (3.14 * d)) } else { "Chyba".into() };
                }
                ui.label(&self.rpm_result);
            } else {
                ui.label("n (ot/min):"); ui.text_edit_singleline(&mut self.feed_n);
                ui.label("z:"); ui.text_edit_singleline(&mut self.feed_z);
                ui.label("fz (mm):"); ui.text_edit_singleline(&mut self.feed_fz);
                if ui.button("Vypočítat").clicked() {
                    let n: f64 = self.feed_n.parse().unwrap_or(0.0);
                    let z: f64 = self.feed_z.parse().unwrap_or(0.0);
                    let fz: f64 = self.feed_fz.parse().unwrap_or(0.0);
                    self.feed_result = format!("{:.1} mm/min", n * z * fz);
                }
                ui.label(&self.feed_result);
            }
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    eframe::run_native(
        "CNC", 
        options, 
        Box::new(|cc| Box::new(CncApp { ..Default::default() }))
    ).unwrap();
}

