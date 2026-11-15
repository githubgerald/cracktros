use eframe::egui;
use crate::app::{Cracktros, SourceFormat};

// Consistent button component
fn tool_button(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let button = egui::Button::new(label)
        .selected(selected)
        .corner_radius(egui::CornerRadius::same(5));

    ui.add_sized([ui.available_width(), 36.0], button)
}

// Draws the application interface
pub fn draw_app(app: &mut Cracktros, ui: &mut egui::Ui) {

        // Top header
        egui::Panel::top("header").show(ui, |ui| {
            ui.horizontal(|ui| {
                // Home button
                let home_button = egui::Button::new(crate::config::SECTION_HOME)
                    .min_size(egui::vec2(65.0, 34.0))
                    .corner_radius(egui::CornerRadius::same(4));

                if ui.add(home_button).clicked() {
                    app.selected_source = None;
                    app.selected_output = None;
                }

                ui.separator();

                // App title
                ui.vertical(|ui| {
                    ui.heading(crate::config::APP_TITLE);
                    ui.label(crate::config::APP_SUBTITLE);
                });
            });
        });

        // Left sidebar
        egui::Panel::left("sidebar")
            .default_size(180.0)
            .show(ui, |ui| {
                ui.add_space(8.0);

                ui.heading(crate::config::SECTION_TOOLS); 
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                // Creates buttons for each tool category
                for tool in [
                    SourceFormat::Png,
                    SourceFormat::Jpg,
                    SourceFormat::Ico,
                    SourceFormat::Webp,
                    SourceFormat::Compressor,
                ] {
                    // Check whether this category is selected
                    let selected = app.selected_source == Some(tool);

                    if tool_button(ui, tool.label(), selected).clicked() {
                        app.selected_source = Some(tool);
                        app.selected_output = tool.default_output();
                    }
                }
            });

        // Main Content that displays a different interface depending on selection
        egui::CentralPanel::default().show(ui, |ui| {
            // Checks the current selection and displays it
            match app.selected_source {
                // Intro screen, this is the default content without a selection
                None => {
                    ui.heading(crate::config::HOME_GREETING);
                    ui.label(crate::config::APP_VERSION); 
                    ui.separator();
                    ui.label(crate::config::HOME_BODY);
                }

                // Compressor screen static content
                Some(SourceFormat::Compressor) => {
                    ui.heading(crate::config::COMPRESSOR_TITLE);
                    ui.label(crate::config::COMPRESSOR_SUBTITLE);
                    ui.separator();
                    ui.label(crate::config::COMPRESSOR_BODY);
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
                                app.selected_output
                                    .map(|output| output.label())
                                    .unwrap_or("Select format"),
                            )
                            .show_ui(ui, |ui| {
                                // Lists the available output formats.
                                for output in source.outputs() {
                                    ui.selectable_value(
                                        &mut app.selected_output,
                                        Some(*output),
                                        output.label(),
                                    );
                                }
                            });
                    });

                    ui.add_space(12.0);
                    ui.separator();

                    // Displays the currently selected conversion.
                    if let Some(output) = app.selected_output {
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