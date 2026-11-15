mod app;
mod config;
mod layout;
mod theme;

use eframe::egui;
fn main() -> eframe::Result {
    // Uses config.rs properties for the window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([theme::WINDOW_WIDTH, theme::WINDOW_HEIGHT])
            .with_min_inner_size([theme::WINDOW_WIDTH, theme::WINDOW_HEIGHT])
            .with_max_inner_size([theme::WINDOW_WIDTH, theme::WINDOW_HEIGHT])
            .with_decorations(false)
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        config::APP_TITLE,
        // Window Settings
        options,
        // Creates the app
        Box::new(|_cc| Ok(Box::new(app::Cracktros::default()))),
    )
}
