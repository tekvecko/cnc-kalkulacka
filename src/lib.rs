use eframe::egui;
use log::info;

struct CncApp {
    // Jednoduchá data, která se neukládají
    text: String,
}

impl Default for CncApp {
    fn default() -> Self {
        Self {
            text: "Funguje to!".to_owned(),
        }
    }
}

impl eframe::App for CncApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("CNC Kalkulačka");
            ui.label("Pokud toto čteš, grafika (OpenGL) naskočila.");
            ui.separator();
            ui.text_edit_singleline(&mut self.text);
        });
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));
    
    // DEFINITIVNÍ VYPNUTÍ VULKANU
    let options = NativeOptions {
        renderer: eframe::Renderer::Glow, // Vynutíme OpenGL
        ..Default::default()
    };
    
    // Spuštění bez persistence (aby nenačítal staré chyby)
    eframe::run_native(
        "CNC Kalkulačka",
        options,
        Box::new(|_cc| Box::new(CncApp::default())),
    ).unwrap();
}
