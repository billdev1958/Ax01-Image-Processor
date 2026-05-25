use egui::{
    Align2, Color32, CornerRadius, CursorIcon, FontId, Rect, Sense, Stroke, StrokeKind,
    TextureHandle, Ui, Vec2,
};
use crate::ui::theme::{OBSIDIAN_1, TEXT};
use crate::ui::widgets::checkerboard::draw_transparency_background;

const ZOOM_MIN: f32 = 0.25;
const ZOOM_MAX: f32 = 4.0;
const ZOOM_STEP: f32 = 0.25;

pub fn image_pane(
    ui: &mut Ui,
    size: Vec2,
    label: &str,
    texture: Option<&TextureHandle>,
    zoom: &mut f32,
    pan: &mut Vec2,
) {
    let (rect, pane_resp) = ui.allocate_exact_size(size, Sense::click_and_drag());

    draw_transparency_background(ui, rect);

    let mut is_pannable = false;
    if let Some(tex) = texture {
        let tex_size = tex.size_vec2();
        let base = (rect.width() / tex_size.x).min(rect.height() / tex_size.y);
        let scale = base * *zoom;
        let img_size = tex_size * scale;
        is_pannable = img_size.x > rect.width() || img_size.y > rect.height();

        if !is_pannable {
            *pan = Vec2::ZERO;
        }

        let img_center = rect.center() + *pan;
        let img_rect = Rect::from_center_size(img_center, img_size);
        let uv = Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
        ui.painter()
            .with_clip_rect(rect)
            .image(tex.id(), img_rect, uv, Color32::WHITE);
    }

    ui.painter().rect_stroke(
        rect,
        CornerRadius::ZERO,
        Stroke::new(1.0, Color32::from_gray(40)),
        StrokeKind::Inside,
    );

    draw_label_badge(ui, rect, label);
    draw_zoom_chip(ui, rect, label, zoom);

    if pane_resp.hovered() {
        let scroll_y = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_y >= 1.0 {
            *zoom = (*zoom + ZOOM_STEP).min(ZOOM_MAX);
        } else if scroll_y <= -1.0 {
            *zoom = (*zoom - ZOOM_STEP).max(ZOOM_MIN);
        }
    }

    if pane_resp.dragged() && is_pannable {
        *pan += pane_resp.drag_delta();
    }

    if pane_resp.hovered() && is_pannable {
        let icon = if pane_resp.dragged() {
            CursorIcon::Grabbing
        } else {
            CursorIcon::Grab
        };
        ui.ctx().set_cursor_icon(icon);
    }
}

fn draw_label_badge(ui: &mut Ui, rect: Rect, label: &str) {
    let font = FontId::proportional(11.0);
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), font.clone(), TEXT);
    let pad = Vec2::new(10.0, 5.0);
    let badge_size = galley.size() + pad * 2.0;
    let badge_pos = rect.min + Vec2::new(12.0, 12.0);
    let badge_rect = Rect::from_min_size(badge_pos, badge_size);

    ui.painter()
        .rect_filled(badge_rect, CornerRadius::same(3u8), OBSIDIAN_1);
    ui.painter().rect_stroke(
        badge_rect,
        CornerRadius::same(3u8),
        Stroke::new(0.5, Color32::from_gray(55)),
        StrokeKind::Inside,
    );
    ui.painter().text(
        badge_rect.center(),
        Align2::CENTER_CENTER,
        label,
        font,
        TEXT,
    );
}

fn draw_zoom_chip(ui: &mut Ui, rect: Rect, label: &str, zoom: &mut f32) {
    let chip_w = 110.0;
    let chip_h = 24.0;
    let btn_w = 28.0;

    let chip_pos = egui::pos2(rect.right() - 12.0 - chip_w, rect.top() + 12.0);
    let chip_rect = Rect::from_min_size(chip_pos, Vec2::new(chip_w, chip_h));

    let minus_rect = Rect::from_min_size(chip_rect.min, Vec2::new(btn_w, chip_h));
    let plus_rect = Rect::from_min_size(
        egui::pos2(chip_rect.right() - btn_w, chip_rect.top()),
        Vec2::new(btn_w, chip_h),
    );
    let pct_rect = Rect::from_min_max(
        egui::pos2(minus_rect.right(), chip_rect.top()),
        egui::pos2(plus_rect.left(), chip_rect.bottom()),
    );

    let id_root = ui.id().with("zoom").with(label);
    let minus_resp = ui.interact(minus_rect, id_root.with("minus"), Sense::click());
    let plus_resp = ui.interact(plus_rect, id_root.with("plus"), Sense::click());

    let painter = ui.painter();
    painter.rect_filled(chip_rect, CornerRadius::same(3u8), OBSIDIAN_1);
    painter.rect_stroke(
        chip_rect,
        CornerRadius::same(3u8),
        Stroke::new(0.5, Color32::from_gray(55)),
        StrokeKind::Inside,
    );

    if minus_resp.hovered() {
        painter.rect_filled(minus_rect, CornerRadius::same(3u8), Color32::from_gray(40));
    }
    if plus_resp.hovered() {
        painter.rect_filled(plus_rect, CornerRadius::same(3u8), Color32::from_gray(40));
    }

    painter.text(
        minus_rect.center(),
        Align2::CENTER_CENTER,
        "−",
        FontId::proportional(14.0),
        TEXT,
    );
    painter.text(
        pct_rect.center(),
        Align2::CENTER_CENTER,
        format!("{:.0}%", *zoom * 100.0),
        FontId::proportional(11.0),
        TEXT,
    );
    painter.text(
        plus_rect.center(),
        Align2::CENTER_CENTER,
        "+",
        FontId::proportional(14.0),
        TEXT,
    );

    if minus_resp.clicked() {
        *zoom = (*zoom - ZOOM_STEP).max(ZOOM_MIN);
    }
    if plus_resp.clicked() {
        *zoom = (*zoom + ZOOM_STEP).min(ZOOM_MAX);
    }
}
