use eframe::egui;
fn main() -> eframe::Result { 
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cracktros の コンバージョン", // Window titles
        options, // Window Settings
        Box::new(|_cc| Ok(Box::new(Cracktros::default()))), // Creates the app
    )
}

// Defines the main app
#[derive(Default)]
struct Cracktros;

// Defines how it updates and displays the interface, connects to the eframe GUI
impl eframe::App for Cracktros {

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {   // Draws the interface

        // Top header
        egui::Panel::top("header").show(ui, |ui| {
            ui.heading("Cracktros の コンバージョン");   // Title
            ui.label("Image conversion utilities"); // Subtitle
        });

        // Left sidebar
        egui::Panel::left("sidebar")
            .default_size(180.0)
            .show(ui, |ui| {
                ui.heading("ツール"); // Tools section title (translates to "Tools")
                ui.separator();

                if ui.button("PNG -> ICO").clicked() {

                }
                if ui.button("ICO -> PNG").clicked() {

                }
                if ui.button("PNG -> WebP").clicked() {

                }
                if ui.button("JPG -> PNG").clicked() {

                }
            });
        
    // Main Content
    egui::CentralPanel::default().show(ui, |ui| {
        ui.heading("どうも"); // Details section title (translates: domo means hello)
        ui.label("バージョン 0.1"); // Version subtitle
        ui.separator();
        ui.label("\"セレクション！\""); // Character dialogue
        });
    }
}

