use eframe::egui;

// Struktura držící stav aplikace (to, co je napsáno v políčkách)
#[derive(Default)]
pub struct CncApp {
    // Otáčky
    rpm_vc: String,
    rpm_d: String,
    rpm_result: String,

    // Posuv
    feed_n: String,
    feed_z: String,
    feed_fz: String,
    feed_result: String,
    
    // Aktivní záložka
    active_tab: usize,
}

impl CncApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// Zde se kreslí uživatelské rozhraní
impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka");
            ui.separator();

            // Přepínač záložek
            ui.horizontal(|ui| {
                if ui.button("Otáčky").clicked() { self.active_tab = 0; }
                if ui.button("Posuv").clicked() { self.active_tab = 1; }
            });
            ui.separator();

            match self.active_tab {
                0 => {
                    ui.label("Výpočet otáček (n)");
                    ui.horizontal(|ui| {
                        ui.label("Vc (m/min):");
                        ui.text_edit_singleline(&mut self.rpm_vc);
                    });
                    ui.horizontal(|ui| {
                        ui.label("D (mm):");
                        ui.text_edit_singleline(&mut self.rpm_d);
                    });

                    if ui.button("Vypočítat").clicked() {
                        let vc: f64 = self.rpm_vc.parse().unwrap_or(0.0);
                        let d: f64 = self.rpm_d.parse().unwrap_or(0.0);
                        if d != 0.0 {
                            let n = (vc * 1000.0) / (3.14159 * d);
                            self.rpm_result = format!("{:.0} ot/min", n);
                        } else {
                            self.rpm_result = "Chyba: D je 0".to_string();
                        }
                    }
                    ui.label(egui::RichText::new(&self.rpm_result).size(20.0).strong());
                }
                1 => {
                    ui.label("Výpočet posuvu (Vf)");
                    ui.horizontal(|ui| {
                        ui.label("n (ot/min):");
                        ui.text_edit_singleline(&mut self.feed_n);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Počet zubů (z):");
                        ui.text_edit_singleline(&mut self.feed_z);
                    });
                    ui.horizontal(|ui| {
                        ui.label("fz (mm/zub):");
                        ui.text_edit_singleline(&mut self.feed_fz);
                    });

                    if ui.button("Vypočítat").clicked() {
                        let n: f64 = self.feed_n.parse().unwrap_or(0.0);
                        let z: f64 = self.feed_z.parse().unwrap_or(0.0);
                        let fz: f64 = self.feed_fz.parse().unwrap_or(0.0);
                        let vf = n * z * fz;
                        self.feed_result = format!("{:.1} mm/min", vf);
                    }
                    ui.label(egui::RichText::new(&self.feed_result).size(20.0).strong());
                }
                _ => {}
            }
        });
    }
}

// Vstupní bod pro Android
#[cfg(target_os = "android")]
use eframe::android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CNC Kalkulačka",
        options,
        Box::new(|cc| Box::new(CncApp::new(cc))),
    ).unwrap();
}

