use crate::app::{Cracktros, SourceFormat};
use crate::config;
use crate::theme;
use eframe::egui;

// Fills a rectangle with artwork, scaled to cover it and cropped to fit.
// Draws a placeholder block if there is no image.
fn draw_art(ui: &egui::Ui, rect: egui::Rect, art: Option<&egui::TextureHandle>) {
    let painter = ui.painter_at(rect);

    let Some(texture) = art else {
        painter.rect_filled(rect, 0.0, theme::COLOUR_PANEL_LIGHT);
        return;
    };

    let image_size = texture.size_vec2();
    let image_aspect = image_size.x / image_size.y;
    let target_aspect = rect.width() / rect.height();

    // Fraction of the image that stays visible on each axis
    let (visible_x, visible_y) = if image_aspect > target_aspect {
        (target_aspect / image_aspect, 1.0) // wider than the target, crop the sides
    } else {
        (1.0, image_aspect / target_aspect) // taller than the target, crop top and bottom
    };

    // The visible part of the texture, positioned by the focus point
    let min_x = (1.0 - visible_x) * theme::ART_FOCUS_X;
    let min_y = (1.0 - visible_y) * theme::ART_FOCUS_Y;
    let uv = egui::Rect::from_min_max(
        egui::pos2(min_x, min_y),
        egui::pos2(min_x + visible_x, min_y + visible_y),
    );

    painter.image(texture.id(), rect, uv, egui::Color32::WHITE);
}

// Consistent button component
fn tool_button(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
    let button = egui::Button::new(label)
        .selected(selected)
        .corner_radius(egui::CornerRadius::same(theme::BUTTON_RADIUS));

    ui.add_sized([ui.available_width(), theme::TOOL_BUTTON_HEIGHT], button)
}

// 1px border strips along one edge of a rectangle, drawn inside it
fn top_edge(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(rect.min, egui::vec2(rect.width(), theme::BORDER_THICKNESS))
}

fn bottom_edge(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(rect.left(), rect.bottom() - theme::BORDER_THICKNESS),
        egui::vec2(rect.width(), theme::BORDER_THICKNESS),
    )
}

fn right_edge(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(rect.right() - theme::BORDER_THICKNESS, rect.top()),
        egui::vec2(theme::BORDER_THICKNESS, rect.height()),
    )
}

// Title bar button, only visible when hovered or pressed so it blends into the bar
fn title_button(
    ui: &egui::Ui,
    rect: egui::Rect,
    id: &str,
    hover_fill: egui::Color32,
) -> egui::Response {
    let response = ui.interact(rect, ui.scope_id().with(id), egui::Sense::click());

    if response.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 0.0, theme::COLOUR_ACCENT);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, hover_fill);
    }

    response
}

// Text with a dark copy behind it so it stays readable over any artwork.
// Returns the text's area so the next line can sit below it.
fn banner_text(
    painter: &egui::Painter,
    pos: egui::Pos2,
    text: &str,
    size: f32,
    colour: egui::Color32,
) -> egui::Rect {
    let font = egui::FontId::proportional(size);
    let shadow = egui::vec2(
        theme::BANNER_TEXT_SHADOW_OFFSET,
        theme::BANNER_TEXT_SHADOW_OFFSET,
    );

    painter.text(
        pos + shadow,
        egui::Align2::LEFT_TOP,
        text,
        font.clone(),
        theme::COLOUR_BACKGROUND,
    );
    painter.text(pos, egui::Align2::LEFT_TOP, text, font, colour)
}

// Draws the application interface
pub fn draw_app(app: &mut Cracktros, ui: &mut egui::Ui) {
    let window = ui.max_rect();

    // Each section is cut from the window's rectangle, so they touch with no gaps
    let title_rect = egui::Rect::from_min_size(
        window.min,
        egui::vec2(window.width(), theme::TITLE_BAR_HEIGHT),
    );
    let banner_rect = egui::Rect::from_min_size(
        title_rect.left_bottom(),
        egui::vec2(window.width(), theme::BANNER_HEIGHT),
    );
    let status_rect = egui::Rect::from_min_max(
        egui::pos2(window.left(), window.bottom() - theme::STATUS_HEIGHT),
        window.max,
    );

    // The workspace and content area take whatever space is left
    let workspace_rect =
        egui::Rect::from_min_max(banner_rect.left_bottom(), status_rect.right_top());
    let sidebar_rect = egui::Rect::from_min_size(
        workspace_rect.min,
        egui::vec2(
            window.width() * theme::SIDEBAR_WIDTH,
            workspace_rect.height(),
        ),
    );
    let content_rect = egui::Rect::from_min_max(sidebar_rect.right_top(), workspace_rect.max);

    draw_title_bar(app, ui, title_rect);
    draw_banner(app, ui, banner_rect);
    draw_sidebar(app, ui, sidebar_rect);
    draw_content(app, ui, content_rect);
    draw_status_bar(app, ui, status_rect);

    // Border around the whole window
    ui.painter().rect_stroke(
        window,
        0.0,
        egui::Stroke::new(theme::BORDER_THICKNESS, theme::COLOUR_BORDER),
        egui::StrokeKind::Inside,
    );
}

// Top bar
fn draw_title_bar(app: &mut Cracktros, ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::COLOUR_PANEL);

    let window_button = egui::vec2(theme::WINDOW_BUTTON_WIDTH, theme::WINDOW_BUTTON_HEIGHT);
    let close_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - window_button.x, rect.top()),
        window_button,
    );
    let minimise_rect = close_rect.translate(egui::vec2(-window_button.x, 0.0));
    let home_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(theme::HOME_BUTTON_SIZE, theme::TITLE_BAR_HEIGHT),
    );

    // The empty space between the buttons drags the window
    let drag_rect = egui::Rect::from_min_max(home_rect.right_top(), minimise_rect.left_bottom());
    let drag = ui.interact(
        drag_rect,
        ui.scope_id().with("title_bar_drag"),
        egui::Sense::click_and_drag(),
    );
    if drag.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    // Home button, de-selects options and goes back to the intro screen
    let home = title_button(ui, home_rect, "home_button", theme::COLOUR_PANEL_LIGHT);
    painter.text(
        home_rect.center(),
        egui::Align2::CENTER_CENTER,
        "✦",
        egui::FontId::proportional(theme::HOME_ICON_SIZE),
        theme::COLOUR_ACCENT,
    );
    if home.clicked() {
        app.selected_source = None;
        app.selected_output = None;
    }

    painter.text(
        egui::pos2(home_rect.right() + theme::TITLE_GAP, rect.center().y),
        egui::Align2::LEFT_CENTER,
        format!("{} v{}", config::APP_TITLE, config::APP_VERSION),
        egui::FontId::proportional(theme::BODY_SIZE),
        theme::COLOUR_TEXT,
    );

    let minimise = title_button(
        ui,
        minimise_rect,
        "minimise_button",
        theme::COLOUR_PANEL_LIGHT,
    );
    let close = title_button(ui, close_rect, "close_button", theme::COLOUR_ACCENT_PINK);

    // Icons are drawn as lines so they don't depend on font symbols
    let icon = egui::Stroke::new(theme::WINDOW_ICON_THICKNESS, theme::COLOUR_TEXT);
    let half = theme::WINDOW_ICON_SIZE / 2.0;

    let centre = minimise_rect.center();
    painter.line_segment(
        [
            centre - egui::vec2(half, 0.0),
            centre + egui::vec2(half, 0.0),
        ],
        icon,
    );

    let centre = close_rect.center();
    painter.line_segment(
        [
            centre + egui::vec2(-half, -half),
            centre + egui::vec2(half, half),
        ],
        icon,
    );
    painter.line_segment(
        [
            centre + egui::vec2(half, -half),
            centre + egui::vec2(-half, half),
        ],
        icon,
    );

    if minimise.clicked() {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
    if close.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    painter.rect_filled(bottom_edge(rect), 0.0, theme::COLOUR_BORDER);
}

// Top banner
fn draw_banner(app: &Cracktros, ui: &egui::Ui, rect: egui::Rect) {
    draw_art(ui, rect, app.art_banner.as_ref());

    let painter = ui.painter_at(rect);
    let origin = rect.min + egui::vec2(theme::BANNER_TEXT_OFFSET_X, theme::BANNER_TEXT_OFFSET_Y);

    let title = banner_text(
        &painter,
        origin,
        config::APP_TITLE,
        theme::BANNER_TITLE_SIZE,
        theme::COLOUR_BANNER_TITLE,
    );
    banner_text(
        &painter,
        egui::pos2(origin.x, title.bottom() + theme::BANNER_TEXT_GAP),
        config::APP_SUBTITLE,
        theme::BANNER_SUBTITLE_SIZE,
        theme::COLOUR_BANNER_SUBTITLE,
    );

    painter.rect_filled(bottom_edge(rect), 0.0, theme::COLOUR_BORDER);
}

// Left sidebar
fn draw_sidebar(app: &mut Cracktros, ui: &mut egui::Ui, rect: egui::Rect) {
    const TOOLS: [SourceFormat; 5] = [
        SourceFormat::Png,
        SourceFormat::Jpg,
        SourceFormat::Ico,
        SourceFormat::Webp,
        SourceFormat::Compressor,
    ];

    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::COLOUR_PANEL);

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect.shrink(theme::PANEL_PADDING))
            .layout(egui::Layout::top_down(egui::Align::Min)),
        |ui| {
            // Sidebar title
            ui.label(
                egui::RichText::new(config::SECTION_TOOLS)
                    .size(theme::HEADING_SIZE)
                    .strong(),
            );
            ui.add_space(theme::HEADING_GAP);
            ui.separator();
            ui.add_space(theme::HEADING_GAP);

            // Tool selection
            ui.spacing_mut().item_spacing.y = theme::TOOL_BUTTON_GAP;

            for tool in TOOLS {
                let selected = app.selected_source == Some(tool);

                if tool_button(ui, tool.label(), selected).clicked() {
                    app.selected_source = Some(tool);
                    app.selected_output = tool.default_output();
                }
            }
        },
    );

    // Artwork fills the space below the last button
    let art_height = rect.width() / theme::SIDEBAR_ART_ASPECT;
    let art_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left(), rect.bottom() - art_height),
        rect.max,
    );
    draw_art(ui, art_rect, app.art_sidebar.as_ref());
    painter.rect_filled(top_edge(art_rect), 0.0, theme::COLOUR_BORDER);

    painter.rect_filled(right_edge(rect), 0.0, theme::COLOUR_BORDER);
}

// Main content
fn draw_content(app: &mut Cracktros, ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::COLOUR_BACKGROUND);

    // Artwork goes behind a dark layer so the text on top stays readable
    draw_art(ui, rect, app.art_content.as_ref());
    painter.rect_filled(
        rect,
        0.0,
        theme::COLOUR_BACKGROUND.gamma_multiply(theme::CONTENT_ART_DIM),
    );

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect.shrink(theme::PANEL_PADDING))
            .layout(egui::Layout::top_down(egui::Align::Min)),
        |ui| {
            // Main content title
            let section_title = match app.selected_source {
                None => config::SECTION_HOME,
                Some(SourceFormat::Compressor) => config::COMPRESSOR_TITLE,
                Some(source) => source.label(),
            };

            ui.label(
                egui::RichText::new(section_title)
                    .size(theme::HEADING_SIZE)
                    .strong(),
            );
            ui.add_space(theme::HEADING_GAP);
            ui.separator();
            ui.add_space(theme::CONTENT_GAP);

            // Display selected tool
            match app.selected_source {
                None => {
                    ui.heading(config::HOME_GREETING);

                    ui.label(format!("バージョン {}", config::APP_VERSION));

                    ui.add_space(theme::TITLE_GAP);
                    ui.label(egui::RichText::new(config::HOME_BODY).size(theme::BODY_SIZE));
                }

                Some(SourceFormat::Compressor) => {
                    ui.label(
                        egui::RichText::new(config::COMPRESSOR_SUBTITLE)
                            .size(theme::BODY_SIZE)
                            .color(theme::COLOUR_TEXT_MUTED),
                    );
                    ui.add_space(theme::TITLE_GAP);
                    ui.label(config::COMPRESSOR_BODY);
                }

                Some(source) => {
                    ui.label("IMAGE CONVERSION");
                    ui.add_space(theme::TITLE_GAP);

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

                    ui.add_space(theme::CONTENT_GAP);

                    if let Some(output) = app.selected_output {
                        ui.label(format!(
                            "Selected conversion: {} → {}",
                            source.label(),
                            output.label()
                        ));
                    }
                }
            }
        },
    );
}

// Bottom status bar
fn draw_status_bar(app: &Cracktros, ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::COLOUR_PANEL);
    painter.rect_filled(top_edge(rect), 0.0, theme::COLOUR_BORDER);

    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect.shrink2(egui::vec2(theme::PANEL_PADDING, 0.0)))
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = theme::STATUS_GAP;

            ui.label(config::SECTION_STATUS);
            ui.colored_label(theme::COLOUR_SUCCESS, "●");
            ui.label(app.status.label());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("///").color(theme::COLOUR_ACCENT));
            });
        },
    );
}