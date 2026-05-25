use egui::{Button, Color32, Response, Stroke, Ui, Vec2};
use crate::ui::theme::{JADE, OBSIDIAN_2};

pub fn jade_button(ui: &mut Ui, label: &str) -> Response {
    ui.add(
        Button::new(label)
            .fill(Color32::from_rgb(0, 45, 60))
            .stroke(Stroke::new(1.0, JADE))
            .min_size(Vec2::new(110.0, 32.0)),
    )
}

pub fn normal_button(ui: &mut Ui, label: &str) -> Response {
    ui.add(
        Button::new(label)
            .fill(OBSIDIAN_2)
            .stroke(Stroke::new(1.0, Color32::from_gray(60)))
            .min_size(Vec2::new(110.0, 32.0)),
    )
}
