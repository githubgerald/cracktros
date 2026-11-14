use eframe::egui;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default(); // Uses default windows setting from eframe
    eframe::run_native(

        // Window titles
        "Cracktros の コンバージョン",   

        // Window Settings                   
        options,          

        // Creates the app                                  
        Box::new(|_cc| Ok(Box::new(Cracktros::default()))), 
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
    fn default_output(self) -> Option<OutputFormat> {
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
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Draws the interface

        // Top header
        egui::Panel::top("header").show(ui, |ui| {
            ui.horizontal(|ui| {
                // Home button
                if ui.button("CRACKTROS").clicked() {
                    self.selected_source = None;
                    self.selected_output = None;
                }

                ui.separator();

                // App title
                ui.vertical(|ui| {
                    ui.heading("Cracktros の コンバージョン"); // Title
                    ui.label("Image conversion utilities"); // Subtitle
                });
            });
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
                ] {
                    // Check whether this category is selected
                    let is_selected = self.selected_source == Some(tool);

                    // Create a button that will highlight when selected
                    let response = ui.add_sized(
                        [ui.available_width(), 36.0],
                        egui::Button::new(tool.label()).selected(is_selected),
                    );

                    // Updates category when clicked
                    if response.clicked() {
                        // Updates the selected category when clicked
                        self.selected_source = Some(tool);
                        self.selected_output = tool.default_output();
                    }
                    ui.add_space(4.0);
                }
            });

        // Main Content that displays a different interface depending on selection
        egui::CentralPanel::default().show(ui, |ui| {
            // Checks the current selection and displays it
            match self.selected_source {
                // Intro screen, this is the default content without a selection
                None => {
                    ui.heading("どうも"); // (translation: domo means hello)
                    ui.label("バージョン 0.2"); // Version subtitle
                    ui.separator();
                    ui.label("\"セレクション！\""); // Character dialogue
                }

                // Compressor screen static content
                Some(SourceFormat::Compressor) => {
                    ui.heading("コンプレッサー"); // (translation: compressor)
                    ui.label("画像圧縮ユーティリティ"); // (translation: image compression utility)
                    ui.separator();
                    ui.label("Settings");
                }

                // Displays the selected input format and available output formats.
                Some(source) => {
                    ui.label("IMAGE CONVERSION");
                    ui.add_space(8.0);

                    // Displays the input format, arrow and output format dropdown.
                    ui.horizontal(|ui| {
                        ui.heading(source.label());
                        ui.heading("→");

                        egui::ComboBox::from_label("Convert to")
                            .selected_text(
                                self.selected_output
                                    .map(|output| output.label())
                                    .unwrap_or("Select format"),
                            )
                            .show_ui(ui, |ui| {
                                // Lists the available output formats.
                                for output in source.outputs() {
                                    ui.selectable_value(
                                        &mut self.selected_output,
                                        Some(*output),
                                        output.label(),
                                    );
                                }
                            });
                    });

                    ui.add_space(12.0);
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
