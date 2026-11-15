use eframe::egui;

// Defines the colours and appearance used throughout the interface
pub fn configure_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // Background colours
    visuals.panel_fill = egui::Color32::from_rgb(17, 13, 29);
    visuals.window_fill = egui::Color32::from_rgb(27, 20, 43);
    visuals.extreme_bg_color = egui::Color32::from_rgb(12, 9, 21);
    visuals.faint_bg_color = egui::Color32::from_rgb(37, 27, 57);

    // Default text colour
    visuals.override_text_color = Some(egui::Color32::from_rgb(229, 221, 245));

    // Normal buttons
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(39, 29, 59);

    // Hovered buttons
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(62, 43, 91);

    // Pressed buttons
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(83, 53, 119);

    // Selected elements
    visuals.selection.bg_fill = egui::Color32::from_rgb(103, 65, 157);

    // Button borders
    visuals.widgets.inactive.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(65, 48, 91));

    visuals.widgets.hovered.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(139, 97, 191));

    visuals.widgets.active.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(176, 117, 221));

    // Button corners
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(5);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(5);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(5);

    ctx.set_visuals(visuals);
}