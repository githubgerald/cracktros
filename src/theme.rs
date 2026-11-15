// Store visual styling and appearance settings here.
// Only include values and functions that control how the interface
// looks, not application content, settings or functionality.

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

// Application spacing
pub const APP_PADDING: f32 = 12.0;
pub const SECTION_GAP: f32 = 8.0;
pub const PANEL_PADDING: f32 = 12.0;

// Widget dimensions
pub const TOOL_BUTTON_HEIGHT: f32 = 36.0;
pub const TOOL_BUTTON_GAP: f32 = 4.0;

// Corner radii
pub const BUTTON_RADIUS: u8 = 5;
pub const PANEL_RADIUS: u8 = 6;

// Borders
pub const BORDER_THICKNESS: f32 = 1.0;

// Typography
pub const HEADING_SIZE: f32 = 24.0;
pub const BODY_SIZE: f32 = 14.0;
pub const SMALL_SIZE: f32 = 12.0;

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
