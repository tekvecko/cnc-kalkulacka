use eframe::egui;

pub struct CncApp {
    label: String,
}

impl Default for CncApp {
    fn default() -> Self {
        Self {
            label: "Aplikace běží! (Verze 30)".to_owned(),
        }
    }
}

impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka");
            ui.add_space(20.0);
            ui.label(&self.label);
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;
    eframe::run_native("CNC", options, Box::new(|_| Box::new(CncApp::default()))).unwrap();
}
