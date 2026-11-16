// Mainly for visuals

use eframe::egui;

// ---------- Colours ----------
// Backgrounds
pub const COLOUR_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(12, 9, 21);
pub const COLOUR_PANEL: egui::Color32 = egui::Color32::from_rgb(22, 17, 35);
pub const COLOUR_PANEL_LIGHT: egui::Color32 = egui::Color32::from_rgb(32, 25, 49);

// Accents
pub const COLOUR_ACCENT: egui::Color32 = egui::Color32::from_rgb(160, 90, 255);
pub const COLOUR_ACCENT_BLUE: egui::Color32 = egui::Color32::from_rgb(70, 170, 255);
pub const COLOUR_ACCENT_PINK: egui::Color32 = egui::Color32::from_rgb(255, 90, 180);

// Text
pub const COLOUR_TEXT: egui::Color32 = egui::Color32::from_rgb(235, 230, 245);
pub const COLOUR_TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(145, 135, 165);

// Borders (every frame is a bright outer line and a dimmer inner line)
pub const COLOUR_BORDER: egui::Color32 = egui::Color32::from_rgb(65, 50, 85);
pub const COLOUR_BORDER_LIGHT: egui::Color32 = egui::Color32::from_rgb(125, 105, 200);
pub const COLOUR_BORDER_SHADOW: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);

// Status
pub const COLOUR_SUCCESS: egui::Color32 = egui::Color32::from_rgb(100, 220, 160);

// ---------- Typography ----------
pub const HEADING_SIZE: f32 = 24.0;
pub const BODY_SIZE: f32 = 14.0;

pub const FONT_FAMILY: &str = "Cracktros";
pub const FONT_FAMILY_FALLBACK: &str = "Proportional";

pub const FONT_SIZE_TITLE_BAR: f32 = 14.0;
pub const FONT_SIZE_BANNER_TITLE: f32 = 56.0;
pub const FONT_SIZE_BANNER_SUBTITLE: f32 = 20.0;
pub const FONT_SIZE_SECTION_TITLE: f32 = 18.0;
pub const FONT_SIZE_TOOL_BUTTON: f32 = 16.0;
pub const FONT_SIZE_BODY: f32 = 14.0;
pub const FONT_SIZE_SMALL: f32 = 12.0;
pub const FONT_SIZE_STATUS: f32 = 12.0;

pub const FONT_WEIGHT_TITLE: f32 = 1.0;

// ---------- Window ----------
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

// Outer frame around the whole window
pub const WINDOW_BORDER_THICKNESS: f32 = 4.0;
pub const WINDOW_BORDER_WIDTH: f32 = 0.0 * WINDOW_BORDER_THICKNESS + BORDER_GAP; // Padding from the edge of the app

// Minimise and close buttons
pub const WINDOW_BUTTON_WIDTH: f32 = 28.0;
pub const WINDOW_BUTTON_HEIGHT: f32 = 28.0;
pub const WINDOW_ICON_SIZE: f32 = 12.0;
pub const WINDOW_ICON_THICKNESS: f32 = 4.0;

// ---------- Title bar ----------
pub const TITLE_BAR_HEIGHT: f32 = 40.0;
pub const TITLE_BAR_PADDING: f32 = 6.0; // space around and between the bar's buttons
pub const TITLE_GAP: f32 = 12.0; // space between the home button and the title text

// Home button
pub const HOME_BUTTON_SIZE: f32 = 34.0;
pub const HOME_ICON_SIZE: f32 = 18.0;

// ---------- Banner ----------
pub const BANNER_HEIGHT: f32 = 200.0;

// Banner text (offsets origin top-left corner)
pub const BANNER_TITLE_SIZE: f32 = 56.0;
pub const BANNER_SUBTITLE_SIZE: f32 = 20.0;
pub const BANNER_TEXT_OFFSET_X: f32 = 48.0;
pub const BANNER_TEXT_OFFSET_Y: f32 = 52.0;
pub const BANNER_TEXT_GAP: f32 = 4.0;
pub const BANNER_TEXT_SHADOW_OFFSET: f32 = 2.0;
pub const COLOUR_BANNER_TITLE: egui::Color32 = COLOUR_TEXT;
pub const COLOUR_BANNER_SUBTITLE: egui::Color32 = COLOUR_ACCENT_BLUE;

// Banner checkerboard pattern
pub const BANNER_PATTERN_ENABLED: bool = true;
pub const BANNER_PATTERN_CELL_SIZE: f32 = 12.0;
pub const BANNER_PATTERN_WIDTH: f32 = 120.0;
pub const BANNER_PATTERN_HEIGHT: f32 = 96.0;
pub const BANNER_PATTERN_OPACITY: u8 = 35;

// ---------- Sidebar ----------
pub const SIDEBAR_WIDTH: f32 = 0.28; // proportion of the window, the content area takes the rest
pub const SIDEBAR_HEADER_HEIGHT: f32 = 40.0;
pub const SIDEBAR_ART_ASPECT: f32 = 16.0 / 9.0; // width / height of the artwork at the bottom

// Tool buttons
pub const TOOL_BUTTON_HEIGHT: f32 = 60.0;
pub const TOOL_BUTTON_GAP: f32 = 4.0;

// ---------- Content ----------
pub const CONTENT_HEADER_HEIGHT: f32 = 40.0;
pub const CONTENT_GAP: f32 = 16.0; // space between blocks of content
pub const CONTENT_TEXT_GAP: f32 = 12.0; // space between lines of text
pub const CONTENT_ART_DIM: f32 = 0.75; // darkens the artwork so text stays readable (0.0 = clear, 1.0 = black)

// ---------- Status bar ----------
pub const STATUS_HEIGHT: f32 = 32.0;
pub const STATUS_GAP: f32 = 8.0;

// ---------- Panels ----------
pub const PANEL_PADDING: f32 = 12.0; // space between a section's frame and its contents

// Corners
pub const WINDOW_CORNER_CUT: f32 = 10.0;
pub const SECTION_CORNER_CUT: f32 = 6.0;
pub const BUTTON_CORNER_CUT: f32 = 3.0;

// ---------- Borders ----------
pub const BORDER_THICKNESS: f32 = 2.0; // thickness of each line in a frame
pub const BORDER_GAP: f32 = 1.0; // space between the outer and inner line

// Shadows
pub const BORDER_SHADOW_OFFSET_X: f32 = -2.0;
pub const BORDER_SHADOW_OFFSET_Y: f32 = 3.0;
pub const BORDER_SHADOW_THICKNESS: f32 = 3.0;
pub const BORDER_SHADOW_OPACITY: f32 = 0.65;

// ---------- Buttons ----------
pub const BUTTON_RADIUS: u8 = 0; // 0 = sharp corners

// ---------- Artwork ----------
// Folders, a random image is picked from each at start-up
pub const ART_BANNER_DIR: &str = "assets/banner";
pub const ART_SIDEBAR_DIR: &str = "assets/sidebar";
pub const ART_CONTENT_DIR: &str = "assets/content";

// Which part of a cropped image stays visible (0.0 = left/top, 1.0 = right/bottom)
pub const ART_FOCUS_X: f32 = 0.5;
pub const ART_FOCUS_Y: f32 = 0.5;

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
