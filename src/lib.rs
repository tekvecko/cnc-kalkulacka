use eframe::egui;
use log::info;

struct CncApp {
    text: String,
}

impl Default for CncApp {
    fn default() -> Self {
        Self {
            text: "Konečně bez pádu!".to_owned(),
        }
    }
}

impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka");
            ui.label("Pokud toto čteš, 'panic=abort' zafungovalo.");
            ui.text_edit_singleline(&mut self.text);
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;
    
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    
    // Použijeme Glow (OpenGL)
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
