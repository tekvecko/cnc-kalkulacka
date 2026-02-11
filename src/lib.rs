use eframe::egui;
use log::info;

struct CncApp {
    text: String,
}

impl Default for CncApp {
    fn default() -> Self {
        Self {
            text: "Funguje to! (C++ fix)".to_owned(),
        }
    }
}

impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka");
            ui.add_space(20.0);
            ui.label("Pokud toto vidíš, C++ knihovna byla úspěšně načtena.");
            ui.text_edit_singleline(&mut self.text);
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;
    
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    
    let options = NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    
    eframe::run_native(
        "CNC App",
        options,
        Box::new(|_cc| Box::new(CncApp::default())),
    ).unwrap();
}
