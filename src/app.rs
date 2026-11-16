use crate::config;
use crate::{layout, theme};
use eframe::egui;

// Loads a random PNG or JPG from a folder as a texture.
// Returns None if there isn't one, so the layout can draw a placeholder.
fn load_random_art(ctx: &egui::Context, dir: &str) -> Option<egui::TextureHandle> {
    use std::hash::{BuildHasher, Hasher};

    let files: Vec<_> = std::fs::read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.extension()
                .and_then(image::ImageFormat::from_extension)
                .is_some_and(|format| format.reading_enabled())
        })
        .collect();

    if files.is_empty() {
        return None;
    }

    // RandomState has a random seed, so this picks randomly without the rand crate
    let seed = std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish();
    let path = &files[seed as usize % files.len()];

    let mut img = image::open(path).ok()?;

    // Shrinks images bigger than the GPU texture limit
    let max_side = ctx.input(|i| i.max_texture_side) as u32;
    if img.width() > max_side || img.height() > max_side {
        img = img.resize(max_side, max_side, image::imageops::FilterType::Triangle);
    }

    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let colour_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());

    // NEAREST keeps pixel art sharp when scaled
    Some(ctx.load_texture(
        path.to_string_lossy(),
        colour_image,
        egui::TextureOptions::NEAREST,
    ))
}

// Defines the input categories that can be selected in the sidebar
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SourceFormat {
    #[default]
    Png,
    Jpg,
    Ico,
    Webp,
    Compressor,
}

// Defines formats that the selection can be converted into
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Ico,
    Png,
    Jpg,
    Webp,
}

// Application processing status
#[derive(Clone, Copy, Default)]
pub enum AppStatus {
    #[default]
    Ready,
    Loading,
    Converting,
    Complete,
}

impl AppStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Ready => config::STATUS_READY,
            Self::Loading => config::STATUS_LOADING,
            Self::Converting => config::STATUS_CONVERTING,
            Self::Complete => config::STATUS_COMPLETE,
        }
    }
}

// Stores currently selected sidebar category and output
#[derive(Default)]
pub struct Cracktros {
    pub status: AppStatus,
    pub selected_source: Option<SourceFormat>,
    pub selected_output: Option<OutputFormat>,
    pub art_banner: Option<egui::TextureHandle>,
    pub art_sidebar: Option<egui::TextureHandle>,
    pub art_content: Option<egui::TextureHandle>,
    theme_configured: bool, // this checks if the theme has already been applied so it doesnt constantly redraw
} // Checks the value of an input and runs the corresponding option.

// Names and conversions for each category
impl SourceFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::Png => "PNG",
            Self::Jpg => "JPG",
            Self::Ico => "ICO",
            Self::Webp => "WebP",
            Self::Compressor => "COMPRESSOR",
        }
    }

    // Makes a list of outputs for each type of input
    pub fn outputs(self) -> &'static [OutputFormat] {
        match self {
            Self::Png => &[OutputFormat::Ico, OutputFormat::Jpg, OutputFormat::Webp],
            Self::Jpg => &[OutputFormat::Png, OutputFormat::Webp],
            Self::Ico => &[OutputFormat::Png],
            Self::Webp => &[OutputFormat::Png, OutputFormat::Jpg],
            Self::Compressor => &[],
        }
    }

    // Default output when you select an option
    pub fn default_output(self) -> Option<OutputFormat> {
        match self {
            Self::Png => Some(OutputFormat::Ico),
            Self::Jpg => Some(OutputFormat::Png),
            Self::Ico => Some(OutputFormat::Png),
            Self::Webp => Some(OutputFormat::Png),
            Self::Compressor => None,
        }
    }
}

// Display names for the output formats
impl OutputFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::Png => "PNG",
            Self::Jpg => "JPG",
            Self::Ico => "ICO",
            Self::Webp => "WebP",
        }
    }
}

// Defines how it updates and displays the interface, connects to the eframe GUI and draws the application window
impl eframe::App for Cracktros {
    // Draws the interface and applies theme
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Applies the theme once
        if !self.theme_configured {
            theme::configure_theme(ui.ctx());
            self.art_banner = load_random_art(ui.ctx(), theme::ART_BANNER_DIR);
            self.art_sidebar = load_random_art(ui.ctx(), theme::ART_SIDEBAR_DIR);
            self.art_content = load_random_art(ui.ctx(), theme::ART_CONTENT_DIR);
            self.theme_configured = true;
        }
        layout::draw_app(self, ui);
    }
}
