// Store application-specific settings and values here.
// Only include values that define how the application is configured,
// not how its interface looks or behaves visually.

// Application information
pub const APP_TITLE: &str = "Cracktros の コンバージョン";
pub const APP_SUBTITLE: &str = "Image conversion utilities";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Section titles
pub const SECTION_TOOLS: &str = "ツール"; // translation: Tools
pub const SECTION_HOME: &str = "HOME";
pub const SECTION_STATUS: &str = "STATUS";

// Home dialogue
pub const HOME_GREETING: &str = "どうも"; // translation: domo means hello
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
