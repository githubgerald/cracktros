use eframe::egui;

// Application information
pub const APP_TITLE: &str = "Cracktros の コンバージョン";
pub const APP_SUBTITLE: &str = "Image conversion utilities";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Section titles
pub const SECTION_TOOLS: &str = "ツール"; // translation: Tools
pub const SECTION_HOME: &str = "HOME";
pub const SECTION_STATUS: &str = "STATUS";

// Navigation
pub const BUTTON_HOME: &str = "HOME";
pub const BUTTON_BACK: &str = "BACK";
pub const BUTTON_SETTINGS: &str = "SETTINGS";

// Home dialogue
pub const HOME_GREETING: &str = "どうも";  // translation: domo means hello
pub const HOME_BODY: &str = "\"セレクション！\""; 

// Compressor screen
pub const COMPRESSOR_TITLE: &str = "コンプレッサー"; // translation: compressor
pub const COMPRESSOR_SUBTITLE: &str = "画像圧縮ユーティリティ"; // translation: image compression utility
pub const COMPRESSOR_BODY: &str = "Settings";

// Status messages
pub const STATUS_READY: &str = "READY";
pub const STATUS_LOADING: &str = "LOADING";
pub const STATUS_CONVERTING: &str = "CONVERTING";
pub const STATUS_COMPLETE: &str = "COMPLETE";

// Window settings
pub const WINDOW_WIDTH: f32 = 1100.0;
pub const WINDOW_HEIGHT: f32 = 760.0;

// Main section proportions
pub const BANNER_HEIGHT: f32 = 0.30;
pub const WORKSPACE_HEIGHT: f32 = 0.65;
pub const STATUS_HEIGHT: f32 = 0.05;

// Workspace proportions
pub const SIDEBAR_WIDTH: f32 = 0.25;
pub const CONTENT_WIDTH: f32 = 0.75;

// Colours
pub const COLOUR_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(12, 9, 21);
pub const COLOUR_PANEL: egui::Color32 = egui::Color32::from_rgb(22, 17, 35);
pub const COLOUR_PANEL_LIGHT: egui::Color32 = egui::Color32::from_rgb(32, 25, 49);

pub const COLOUR_ACCENT: egui::Color32 = egui::Color32::from_rgb(160, 90, 255);
pub const COLOUR_ACCENT_BLUE: egui::Color32 = egui::Color32::from_rgb(70, 170, 255);
pub const COLOUR_ACCENT_PINK: egui::Color32 = egui::Color32::from_rgb(255, 90, 180);

pub const COLOUR_TEXT: egui::Color32 = egui::Color32::from_rgb(235, 230, 245);
pub const COLOUR_TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(145, 135, 165);
pub const COLOUR_BORDER: egui::Color32 = egui::Color32::from_rgb(65, 50, 85);
pub const COLOUR_SUCCESS: egui::Color32 = egui::Color32::from_rgb(100, 220, 160);

// Layout dimensions
pub const APP_PADDING: f32 = 12.0;
pub const SECTION_GAP: f32 = 8.0;
pub const PANEL_PADDING: f32 = 12.0;

pub const TOOL_BUTTON_HEIGHT: f32 = 36.0;
pub const TOOL_BUTTON_RADIUS: u8 = 5;
pub const PANEL_RADIUS: u8 = 6;

pub const BORDER_THICKNESS: f32 = 1.0;