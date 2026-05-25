use eframe::egui;

pub fn draw_transparency_background(ui: &mut egui::Ui, rect: egui::Rect) {
    let square_size = 16.0;

    let bg_color = egui::Color32::from_rgb(18, 18, 18);
    let square_color = egui::Color32::from_rgb(26, 26, 26);

    ui.painter()
        .rect_filled(rect, egui::CornerRadius::ZERO, bg_color);

    let painter = ui.painter().with_clip_rect(rect);

    let width = rect.width();
    let height = rect.height();

    let cols = (width / square_size).ceil() as i32;
    let rows = (height / square_size).ceil() as i32;

    for row in 0..rows {
        for col in 0..cols {
            if (row + col) % 2 == 1 {
                let min_x = rect.min.x + (col as f32) * square_size;
                let min_y = rect.min.y + (row as f32) * square_size;

                let square_rect = egui::Rect::from_min_size(
                    egui::pos2(min_x, min_y),
                    egui::vec2(square_size, square_size),
                );
                painter.rect_filled(square_rect, egui::CornerRadius::ZERO, square_color);
            }
        }
    }
}
