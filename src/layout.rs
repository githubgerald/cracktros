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

// Draws a double-line frame inside a rectangle: an outer line, a gap, then an inner line
fn draw_frame(
    painter: &egui::Painter,
    rect: egui::Rect,
    thickness: f32,
    outer: egui::Color32,
    inner: egui::Color32,
) {
    painter.rect_stroke(
        rect,
        0.0,
        egui::Stroke::new(thickness, outer),
        egui::StrokeKind::Inside,
    );
}

// The frame used around sections, header strips and buttons
fn draw_section_frame(painter: &egui::Painter, rect: egui::Rect) {
    draw_frame(
        painter,
        rect,
        theme::BORDER_THICKNESS,
        theme::COLOUR_BORDER_LIGHT,
        theme::COLOUR_BORDER,
    );
}

// Framed title bar button, filled only when hovered or pressed
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

    // Frame, with the two lines swapped while pressed so the button looks pushed in
    let (outer, inner) = if response.is_pointer_button_down_on() {
        (theme::COLOUR_BORDER, theme::COLOUR_BORDER_LIGHT)
    } else {
        (theme::COLOUR_BORDER_LIGHT, theme::COLOUR_BORDER)
    };
    draw_frame(ui.painter(), rect, theme::BORDER_THICKNESS, outer, inner);

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
    ui.painter()
        .rect_filled(window, 0.0, theme::COLOUR_BACKGROUND);

    // The sections sit inside the window's own frame
    let inner = window.shrink(theme::WINDOW_BORDER_WIDTH);

    // Neighbouring sections overlap by one line, so they share a single outer line
    let overlap_y = egui::vec2(0.0, theme::BORDER_THICKNESS);
    let overlap_x = egui::vec2(theme::BORDER_THICKNESS, 0.0);

    // Each section is cut from the inner rectangle, so they touch with no gaps
    let title_rect = egui::Rect::from_min_size(
        inner.min,
        egui::vec2(inner.width(), theme::TITLE_BAR_HEIGHT),
    );
    let banner_rect = egui::Rect::from_min_size(
        title_rect.left_bottom() - overlap_y,
        egui::vec2(inner.width(), theme::BANNER_HEIGHT),
    );
    let status_rect = egui::Rect::from_min_max(
        egui::pos2(inner.left(), inner.bottom() - theme::STATUS_HEIGHT),
        inner.max,
    );

    // The workspace and content area take whatever space is left
    let workspace_rect = egui::Rect::from_min_max(
        banner_rect.left_bottom() - overlap_y,
        status_rect.right_top() + overlap_y,
    );
    let sidebar_rect = egui::Rect::from_min_size(
        workspace_rect.min,
        egui::vec2(
            inner.width() * theme::SIDEBAR_WIDTH,
            workspace_rect.height(),
        ),
    );
    let content_rect =
        egui::Rect::from_min_max(sidebar_rect.right_top() - overlap_x, workspace_rect.max);

    draw_title_bar(app, ui, title_rect);
    draw_banner(app, ui, banner_rect);
    draw_sidebar(app, ui, sidebar_rect);
    draw_content(app, ui, content_rect);
    draw_status_bar(app, ui, status_rect);

    // Thicker frame around the whole window
    draw_frame(
        ui.painter(),
        window,
        theme::WINDOW_BORDER_THICKNESS,
        theme::COLOUR_BORDER_LIGHT,
        theme::COLOUR_BORDER,
    );
}

// Top bar
fn draw_title_bar(app: &mut Cracktros, ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::COLOUR_PANEL);

    // Buttons are centred vertically and kept clear of the bar's frame
    let window_button = egui::vec2(theme::WINDOW_BUTTON_WIDTH, theme::WINDOW_BUTTON_HEIGHT);
    let button_top = rect.center().y - window_button.y / 2.0;
    let close_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.right() - theme::TITLE_BAR_PADDING - window_button.x,
            button_top,
        ),
        window_button,
    );
    let minimise_rect = close_rect.translate(egui::vec2(
        -(window_button.x + theme::TITLE_BAR_PADDING),
        0.0,
    ));
    let home_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.left() + theme::TITLE_BAR_PADDING,
            rect.center().y - theme::HOME_BUTTON_SIZE / 2.0,
        ),
        egui::vec2(theme::HOME_BUTTON_SIZE, theme::HOME_BUTTON_SIZE),
    );

    // The empty space between the buttons drags the window
    let drag_rect = egui::Rect::from_min_max(
        egui::pos2(home_rect.right(), rect.top()),
        egui::pos2(minimise_rect.left(), rect.bottom()),
    );
    let drag = ui.interact(
        drag_rect,
        ui.scope_id().with("title_bar_drag"),
        egui::Sense::click_and_drag(),
    );
    if drag.drag_started_by(egui::PointerButton::Primary) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }

    // Home button, de-selects options and goes back to the intro screen
    let home = ui.interact(
        home_rect,
        ui.scope_id().with("home_button"),
        egui::Sense::click(),
        
    );
    if home.is_pointer_button_down_on() {
        painter.rect_filled(home_rect, 0.0, theme::COLOUR_ACCENT);
    } else if home.hovered() {
        painter.rect_filled(home_rect, 0.0, theme::COLOUR_PANEL_LIGHT);
    }
    if home.clicked() {
        app.selected_source = None;
        app.selected_output = None;
    }
    painter.text(
    home_rect.center(),
    egui::Align2::CENTER_CENTER,
    "T",
    egui::FontId::proportional(theme::HOME_ICON_SIZE),
    theme::COLOUR_ACCENT,
    );

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

    draw_section_frame(&painter, rect);
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

    draw_section_frame(&painter, rect);
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

    // Sidebar title in its own framed strip
    let header_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), theme::SIDEBAR_HEADER_HEIGHT),
    );
    painter.text(
        egui::pos2(
            header_rect.left() + theme::PANEL_PADDING,
            header_rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        config::SECTION_TOOLS,
        egui::FontId::proportional(theme::HEADING_SIZE),
        theme::COLOUR_TEXT,
    );
    draw_section_frame(&painter, header_rect);

    // Tool selection starts below the header
    let tools_rect = egui::Rect::from_min_max(
        egui::pos2(
            rect.left() + theme::PANEL_PADDING,
            header_rect.bottom() + theme::PANEL_PADDING,
        ),
        egui::pos2(rect.right() - theme::PANEL_PADDING, rect.bottom()),
    );
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(tools_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
        |ui| {
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

    // Artwork sits at the bottom of the sidebar, cut to a fixed shape
    let art_height = rect.width() / theme::SIDEBAR_ART_ASPECT;
    let art_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left(), rect.bottom() - art_height),
        rect.max,
    );
    draw_art(ui, art_rect, app.art_sidebar.as_ref());
    draw_section_frame(&painter, art_rect);

    draw_section_frame(&painter, rect);
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

    // Main content title in its own framed strip
    let section_title = match app.selected_source {
        None => config::SECTION_HOME,
        Some(SourceFormat::Compressor) => config::COMPRESSOR_TITLE,
        Some(source) => source.label(),
    };
    let header_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), theme::CONTENT_HEADER_HEIGHT),
    );
    painter.rect_filled(header_rect, 0.0, theme::COLOUR_PANEL);
    painter.text(
        egui::pos2(
            header_rect.left() + theme::PANEL_PADDING,
            header_rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        section_title,
        egui::FontId::proportional(theme::HEADING_SIZE),
        theme::COLOUR_TEXT,
    );
    draw_section_frame(&painter, header_rect);

    // The selected tool's interface starts below the header
    let body_rect = egui::Rect::from_min_max(
        egui::pos2(
            rect.left() + theme::PANEL_PADDING,
            header_rect.bottom() + theme::PANEL_PADDING,
        ),
        rect.max - egui::vec2(theme::PANEL_PADDING, theme::PANEL_PADDING),
    );
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(body_rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
        |ui| {
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
    draw_section_frame(&painter, rect);
}

// Bottom status bar
fn draw_status_bar(app: &Cracktros, ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, theme::COLOUR_PANEL);

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
    draw_section_frame(&painter, rect);
}
