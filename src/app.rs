use eframe::egui;
use crate::{layout, theme};

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

// Stores currently selected sidebar category and output
#[derive(Default)]
pub struct Cracktros {
    pub selected_source: Option<SourceFormat>,
    pub selected_output: Option<OutputFormat>,
    theme_configured: bool, // this checks if the theme has already been applied so it doesnt constantly redraw
}

// Names and conversions for each category
impl SourceFormat {
    pub fn label(self) -> &'static str {
        match self {
            // Checks the value of an input and runs the corresponding option.
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
            self.theme_configured = true;
        }   

        layout::draw_app(self, ui);
    }
}