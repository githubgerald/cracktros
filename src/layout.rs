use crate::app::{Cracktros, SourceFormat};
use crate::config;
use eframe::egui;

// Consistent button component
fn tool_button(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let button = egui::Button::new(label)
        .selected(selected)
        .corner_radius(egui::CornerRadius::same(config::TOOL_BUTTON_RADIUS));

    ui.add_sized([ui.available_width(), config::TOOL_BUTTON_HEIGHT], button)
}

// Draws the application interface
pub fn draw_app(app: &mut Cracktros, ui: &mut egui::Ui) {
    let available = ui.available_size();

    // Main interface spacing
    let padding = config::APP_PADDING;
    let gap = config::SECTION_GAP;

    let inner_width = available.x - padding * 2.0;
    let inner_height = available.y - padding * 2.0 - gap * 2.0;

    // Calculate section heights
    let banner_height = inner_height * config::BANNER_HEIGHT;
    let workspace_height = inner_height * config::WORKSPACE_HEIGHT;
    let status_height = inner_height * config::STATUS_HEIGHT;

    // Main application background and padding
    egui::Frame::new()
        .fill(config::COLOUR_BACKGROUND)
        .inner_margin(egui::Margin::same(padding as i8))
        .show(ui, |ui| {
            let inner_size = ui.available_size();
            ui.set_min_size(inner_size);

            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

            // Top banner
            ui.allocate_ui_with_layout(
                egui::vec2(inner_width, banner_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::Frame::new()
                        .fill(config::COLOUR_PANEL)
                        .stroke(egui::Stroke::new(
                            config::BORDER_THICKNESS,
                            config::COLOUR_BORDER,
                        ))
                        .corner_radius(egui::CornerRadius::same(config::PANEL_RADIUS))
                        .inner_margin(egui::Margin::same(config::PANEL_PADDING as i8))
                        .show(ui, |ui| {
                            let size = ui.available_size();
                            ui.set_min_size(size);

                            ui.horizontal(|ui| {
                                // 48x48 sized home button
                                let home_button =
                                    ui.add_sized([48.0, 48.0], egui::Button::new("✦"));

                                // de-selects options and goes back to the intro screen
                                if home_button.clicked() {
                                    app.selected_source = None;
                                    app.selected_output = None;
                                }

                                ui.add_space(12.0);

                                ui.vertical(|ui| {
                                    ui.heading(config::APP_TITLE);
                                    ui.label(config::APP_SUBTITLE);
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(format!("v{}", config::APP_VERSION));
                                    },
                                );
                            });
                        });
                },
            );

            ui.add_space(gap);

            // Middle workspace
            ui.allocate_ui_with_layout(
                egui::vec2(inner_width, workspace_height),
                egui::Layout::left_to_right(egui::Align::Min),
                |ui| {
                    let sidebar_width = (inner_width - gap) * config::SIDEBAR_WIDTH;
                    let content_width = inner_width - sidebar_width - gap;

                    // Left sidebar
                    ui.allocate_ui_with_layout(
                        egui::vec2(sidebar_width, workspace_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::Frame::new()
                                .fill(config::COLOUR_PANEL_LIGHT)
                                .stroke(egui::Stroke::new(
                                    config::BORDER_THICKNESS,
                                    config::COLOUR_BORDER,
                                ))
                                .corner_radius(egui::CornerRadius::same(config::PANEL_RADIUS))
                                .inner_margin(egui::Margin::same(config::PANEL_PADDING as i8))
                                .show(ui, |ui| {
                                    let size = ui.available_size();
                                    ui.set_min_size(size);

                                    // Sidebar title
                                    ui.heading(config::SECTION_TOOLS);
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(8.0);

                                    // Tool selection
                                    for tool in [
                                        SourceFormat::Png,
                                        SourceFormat::Jpg,
                                        SourceFormat::Ico,
                                        SourceFormat::Webp,
                                        SourceFormat::Compressor,
                                    ] {
                                        let selected = app.selected_source == Some(tool);

                                        if tool_button(ui, tool.label(), selected).clicked() {
                                            app.selected_source = Some(tool);
                                            app.selected_output = tool.default_output();
                                        }
                                    }
                                });
                        },
                    );

                    ui.add_space(gap);

                    // Main content
                    ui.allocate_ui_with_layout(
                        egui::vec2(content_width, workspace_height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            egui::Frame::new()
                                .fill(config::COLOUR_BACKGROUND)
                                .stroke(egui::Stroke::new(
                                    config::BORDER_THICKNESS,
                                    config::COLOUR_BORDER,
                                ))
                                .corner_radius(egui::CornerRadius::same(config::PANEL_RADIUS))
                                .inner_margin(egui::Margin::same(config::PANEL_PADDING as i8))
                                .show(ui, |ui| {
                                    let size = ui.available_size();
                                    ui.set_min_size(size);

                                    // Main content title
                                    let section_title = match app.selected_source {
                                        None => config::SECTION_HOME,
                                        Some(SourceFormat::Compressor) => config::COMPRESSOR_TITLE,
                                        Some(source) => source.label(),
                                    };

                                    ui.heading(section_title);
                                    ui.add_space(8.0);
                                    ui.separator();
                                    ui.add_space(16.0);

                                    // Display selected tool
                                    match app.selected_source {
                                        None => {
                                            ui.heading(config::HOME_GREETING);

                                            ui.label(format!("バージョン {}", config::APP_VERSION));

                                            ui.add_space(12.0);
                                            ui.label(config::HOME_BODY);
                                        }

                                        Some(SourceFormat::Compressor) => {
                                            ui.heading(config::COMPRESSOR_TITLE);
                                            ui.label(config::COMPRESSOR_SUBTITLE);
                                            ui.add_space(12.0);
                                            ui.label(config::COMPRESSOR_BODY);
                                        }

                                        Some(source) => {
                                            ui.label("IMAGE CONVERSION");
                                            ui.add_space(12.0);

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
                                                        for output in source.outputs() {
                                                            ui.selectable_value(
                                                                &mut app.selected_output,
                                                                Some(*output),
                                                                output.label(),
                                                            );
                                                        }
                                                    });
                                            });

                                            ui.add_space(16.0);

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
                        },
                    );
                },
            );

            ui.add_space(gap);

            // Bottom status bar
            ui.allocate_ui_with_layout(
                egui::vec2(inner_width, status_height),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    egui::Frame::new()
                        .fill(config::COLOUR_PANEL)
                        .stroke(egui::Stroke::new(
                            config::BORDER_THICKNESS,
                            config::COLOUR_BORDER,
                        ))
                        .corner_radius(egui::CornerRadius::same(config::PANEL_RADIUS))
                        .inner_margin(egui::Margin::same(config::SECTION_GAP as i8))
                        .show(ui, |ui| {
                            let size = ui.available_size();
                            ui.set_min_size(size);

                            ui.horizontal(|ui| {
                                ui.colored_label(config::COLOUR_SUCCESS, "●");

                                ui.label(config::STATUS_READY);

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.label(
                                            egui::RichText::new("///").color(config::COLOUR_ACCENT),
                                        );
                                    },
                                );
                            });
                        });
                },
            );
        });
}
