use eframe::egui;
fn main() -> eframe::Result { 
    let options = eframe::NativeOptions::default(); // Uses default windows setting from eframe
    eframe::run_native(
        "Cracktros の コンバージョン", // Window titles
        options, // Window Settings
        Box::new(|_cc| Ok(Box::new(Cracktros::default()))), // Creates the app
    )
}

// Defines the input categories that can be selected in the sidebar
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum SourceFormat {
    #[default]
    Png,
    Jpg,
    Ico,
    Webp,
    Compressor,
}

// Defines formats that the selection can be converted into
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum OutputFormat {
    #[default]
    Ico,
    Png,
    Jpg,
    Webp,
}

// Names and conversions for each category
impl SourceFormat {
    fn label(self) -> &'static str {
        match self { // Checks the value of an input and runs the corresponding option.
            Self::Png => "PNG",
            Self::Jpg => "JPG",
            Self::Ico => "ICO",
            Self::Webp => "WebP",
            Self::Compressor => "COMPRESSOR",
        }
    }

    // Makes a list of outputs for each type of input
    fn outputs(self) -> &'static [OutputFormat] {
        match self {
            Self::Png => &[OutputFormat::Ico, OutputFormat::Jpg, OutputFormat::Webp],
            Self::Jpg => &[OutputFormat::Png, OutputFormat::Webp],
            Self::Ico => &[OutputFormat::Png],
            Self::Webp => &[OutputFormat::Png, OutputFormat::Jpg],
            Self::Compressor => &[],
        }
    }

    // Default output when you select an option
    fn default_output(self) -> OutputFormat {
        match self {
            Self::Png => OutputFormat::Ico,
            Self::Jpg => OutputFormat::Png,
            Self::Ico => OutputFormat::Png,
            Self::Webp => OutputFormat::Png,
            Self::Compressor => OutputFormat::Png,
        }
    }
}

// Display names for the output formats
impl OutputFormat {
    fn label(self) -> &'static str {
        match self {
            Self::Png => "PNG",
            Self::Jpg => "JPG",
            Self::Ico => "ICO",
            Self::Webp => "WebP",
        }
    }
}

// Stores currently selected sidebar category and output
#[derive(Default)]
struct Cracktros {
    selected_source: Option<SourceFormat>,
    selected_output: Option<OutputFormat>,
}

// Defines how it updates and displays the interface, connects to the eframe GUI and draws the application window
impl eframe::App for Cracktros {

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {   // Draws the interface

        // Top header
        egui::Panel::top("header").show(ui, |ui| {
            ui.heading("Cracktros の コンバージョン");   // Title
            ui.label("Image conversion utilities"); // Subtitle
        });

        // Left sidebar
        egui::Panel::left("sidebar")
            .default_size(180.0)
            .show(ui, |ui| {
                ui.heading("ツール"); // Tools section title (translates to "Tools")
                ui.separator();

                // Creates buttons for each tool category
                for tool in [
                    SourceFormat::Png,
                    SourceFormat::Jpg,
                    SourceFormat::Ico,
                    SourceFormat::Webp,
                    SourceFormat::Compressor,
                ] 
                {
                    if ui.selectable_label(self.selected_source == Some(tool), tool.label(),).clicked() 
                    {
                        // Updates the selected category and resets the output format
                        self.selected_source = Some(tool);
                        self.selected_output = Some(tool.default_output());
                    }
                }
            });
        
    // Main Content that displays a different interface depending on selection
    egui::CentralPanel::default().show(ui, |ui| {

        // Checks the current selection and displays it
        match self.selected_source {

            // Intro screen, this is the default content without a selection
            None => 
            {
                ui.heading("どうも"); // (translation: domo means hello)
                ui.label("バージョン 0.2"); // Version subtitle
                ui.separator();
                ui.label("\"セレクション！\""); // Character dialogue
            }

            // Compressor screen static content
            Some(SourceFormat::Compressor) => 
            {
                ui.heading("コンプレッサー"); // (translation: compressor)
                ui.label("画像圧縮ユーティリティ"); // (translation: image compression utility)
                ui.separator();
                ui.label("Settings");
            }

            // Dynamic conversion screen for the input selections.
            Some(source) => {
                ui.heading(source.label());
                ui.label("Choose an output:");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading(source.label());
                    ui.label("→");

                    egui::ComboBox::from_label("Convert to")
                        .selected_text(
                            self.selected_output
                                .map(|output| output.label())
                                .unwrap_or("Select format"),
                        )
                        .show_ui(ui, |ui| {
                            for output in source.outputs() {
                                ui.selectable_value(
                                    &mut self.selected_output,
                                    Some(*output),
                                    output.label(),
                                );
                            }
                        });
                    });

                    ui.separator();

                    // Displays the currently selected conversion.
                    if let Some(output) = self.selected_output {
                        ui.label(format!(
                            "Selected conversion: {} → {}",
                            source.label(),
                            output.label()
                        ));
                    }
                }
            }
        });
    }
}

