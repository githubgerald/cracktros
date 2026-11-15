// Mainly for visuals

use eframe::egui;

// Colour palette
pub const COLOUR_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(12, 9, 21);
pub const COLOUR_PANEL: egui::Color32 = egui::Color32::from_rgb(22, 17, 35);
pub const COLOUR_PANEL_LIGHT: egui::Color32 = egui::Color32::from_rgb(32, 25, 49);

// Accent colours
pub const COLOUR_ACCENT: egui::Color32 = egui::Color32::from_rgb(160, 90, 255);
pub const COLOUR_ACCENT_BLUE: egui::Color32 = egui::Color32::from_rgb(70, 170, 255);
pub const COLOUR_ACCENT_PINK: egui::Color32 = egui::Color32::from_rgb(255, 90, 180);

// Text colours
pub const COLOUR_TEXT: egui::Color32 = egui::Color32::from_rgb(235, 230, 245);
pub const COLOUR_TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(145, 135, 165);

// Other colours
pub const COLOUR_BORDER: egui::Color32 = egui::Color32::from_rgb(65, 50, 85);
pub const COLOUR_SUCCESS: egui::Color32 = egui::Color32::from_rgb(100, 220, 160);

// Window
pub const WINDOW_WIDTH: f32 = 1100.0;
pub const WINDOW_HEIGHT: f32 = 760.0;

// Main section heights (the workspace takes whatever space is left)
pub const TITLE_BAR_HEIGHT: f32 = 36.0;
pub const BANNER_HEIGHT: f32 = 200.0;
pub const STATUS_HEIGHT: f32 = 36.0;

// Workspace proportions (the content area takes whatever width is left)
pub const SIDEBAR_WIDTH: f32 = 0.25;

// Application spacing
pub const PANEL_PADDING: f32 = 12.0;
pub const TITLE_GAP: f32 = 12.0;
pub const HEADING_GAP: f32 = 8.0;
pub const CONTENT_GAP: f32 = 16.0;
pub const STATUS_GAP: f32 = 8.0;

// Widget dimensions
pub const TOOL_BUTTON_HEIGHT: f32 = 36.0;
pub const TOOL_BUTTON_GAP: f32 = 4.0;
pub const HOME_BUTTON_SIZE: f32 = 36.0;
pub const HOME_ICON_SIZE: f32 = 20.0;
pub const WINDOW_BUTTON_WIDTH: f32 = 36.0;
pub const WINDOW_BUTTON_HEIGHT: f32 = 36.0;
pub const WINDOW_ICON_SIZE: f32 = 12.0;
pub const WINDOW_ICON_THICKNESS: f32 = 2.0;

// Corner radii (0 = sharp)
pub const BUTTON_RADIUS: u8 = 0;

// Borders
pub const BORDER_THICKNESS: f32 = 1.0;

// Typography
pub const HEADING_SIZE: f32 = 24.0;
pub const BODY_SIZE: f32 = 14.0;

// Artwork folders, a random image is picked from each at start-up
pub const ART_BANNER_DIR: &str = "assets/banner";
pub const ART_SIDEBAR_DIR: &str = "assets/sidebar";
pub const ART_CONTENT_DIR: &str = "assets/content";

// Which part of a cropped image stays visible (0.0 = left/top, 1.0 = right/bottom)
pub const ART_FOCUS_X: f32 = 0.5;
pub const ART_FOCUS_Y: f32 = 0.5;

// Darkens the content artwork so text stays readable (0.0 = clear, 1.0 = black)
pub const CONTENT_ART_DIM: f32 = 0.75;

// Banner text (offsets are from the banner's top-left corner)
pub const BANNER_TITLE_SIZE: f32 = 56.0;
pub const BANNER_SUBTITLE_SIZE: f32 = 20.0;
pub const BANNER_TEXT_OFFSET_X: f32 = 48.0;
pub const BANNER_TEXT_OFFSET_Y: f32 = 52.0;
pub const BANNER_TEXT_GAP: f32 = 4.0;
pub const BANNER_TEXT_SHADOW_OFFSET: f32 = 2.0;
pub const COLOUR_BANNER_TITLE: egui::Color32 = COLOUR_TEXT;
pub const COLOUR_BANNER_SUBTITLE: egui::Color32 = COLOUR_ACCENT_BLUE;

// Applies the application theme
pub fn configure_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // Background colours
    visuals.panel_fill = COLOUR_PANEL;
    visuals.window_fill = COLOUR_PANEL;
    visuals.extreme_bg_color = COLOUR_BACKGROUND;
    visuals.faint_bg_color = COLOUR_PANEL_LIGHT;

    // Text
    visuals.override_text_color = Some(COLOUR_TEXT);

    // Selection
    visuals.selection.bg_fill = COLOUR_ACCENT;
    visuals.selection.stroke = egui::Stroke::new(BORDER_THICKNESS, COLOUR_ACCENT_BLUE);

    // Inactive widgets
    visuals.widgets.inactive.bg_fill = COLOUR_PANEL_LIGHT;
    visuals.widgets.inactive.fg_stroke.color = COLOUR_TEXT;
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(BORDER_THICKNESS, COLOUR_BORDER);

    // Hovered widgets
    visuals.widgets.hovered.bg_fill = COLOUR_ACCENT;
    visuals.widgets.hovered.fg_stroke.color = COLOUR_TEXT;
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(BORDER_THICKNESS, COLOUR_ACCENT_BLUE);

    // Active widgets
    visuals.widgets.active.bg_fill = COLOUR_ACCENT;
    visuals.widgets.active.fg_stroke.color = COLOUR_TEXT;
    visuals.widgets.active.bg_stroke = egui::Stroke::new(BORDER_THICKNESS, COLOUR_ACCENT_PINK);

    // Non-interactive widgets
    visuals.widgets.noninteractive.bg_fill = COLOUR_PANEL;
    visuals.widgets.noninteractive.fg_stroke.color = COLOUR_TEXT;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(BORDER_THICKNESS, COLOUR_BORDER);

    // Widget corners
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(BUTTON_RADIUS);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(BUTTON_RADIUS);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(BUTTON_RADIUS);
    visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(BUTTON_RADIUS);

    ctx.set_visuals(visuals);
}
