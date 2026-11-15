mod app;
mod layout;
mod theme;
mod config;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default(); // Uses default windows setting from eframe
    eframe::run_native(
        // Window titles
        config::APP_TITLE,
        // Window Settings
        options,
        // Creates the app
        Box::new(|_cc| Ok(Box::new(app::Cracktros::default()))),
    )
}