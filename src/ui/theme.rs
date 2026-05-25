use egui::{Color32, CornerRadius, Stroke, Visuals};

pub const OBSIDIAN_0: Color32 = Color32::from_rgb(18, 18, 18);
pub const OBSIDIAN_1: Color32 = Color32::from_rgb(26, 26, 26);
pub const OBSIDIAN_2: Color32 = Color32::from_rgb(34, 34, 34);
pub const JADE: Color32 = Color32::from_rgb(0, 229, 255);
pub const TEXT: Color32 = Color32::from_rgb(220, 220, 220);
pub const NAV_ACTIVE: Color32 = Color32::from_rgb(40, 40, 40);

pub fn apply_theme(ctx: &egui::Context) {
    let mut v = Visuals::dark();
    let rnd = CornerRadius::same(4u8);

    v.panel_fill = OBSIDIAN_1;
    v.window_fill = OBSIDIAN_1;
    v.faint_bg_color = OBSIDIAN_0;
    v.extreme_bg_color = OBSIDIAN_0;

    v.selection.bg_fill = Color32::from_rgb(0, 80, 100);
    v.selection.stroke = Stroke::new(1.0, JADE);

    v.widgets.noninteractive.bg_fill = OBSIDIAN_2;
    v.widgets.noninteractive.weak_bg_fill = OBSIDIAN_2;
    v.widgets.noninteractive.bg_stroke = Stroke::new(0.0, Color32::TRANSPARENT);
    v.widgets.noninteractive.corner_radius = rnd;
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);

    v.widgets.inactive.bg_fill = OBSIDIAN_2;
    v.widgets.inactive.weak_bg_fill = OBSIDIAN_2;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_gray(50));
    v.widgets.inactive.corner_radius = rnd;
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);

    v.widgets.hovered.bg_fill = Color32::from_rgb(45, 45, 45);
    v.widgets.hovered.weak_bg_fill = Color32::from_rgb(45, 45, 45);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, JADE);
    v.widgets.hovered.corner_radius = rnd;
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, JADE);

    v.widgets.active.bg_fill = Color32::from_rgb(0, 60, 75);
    v.widgets.active.weak_bg_fill = Color32::from_rgb(0, 60, 75);
    v.widgets.active.bg_stroke = Stroke::new(1.5, JADE);
    v.widgets.active.corner_radius = rnd;
    v.widgets.active.fg_stroke = Stroke::new(1.5, JADE);

    v.widgets.open.bg_fill = OBSIDIAN_2;
    v.widgets.open.weak_bg_fill = OBSIDIAN_2;
    v.widgets.open.corner_radius = rnd;

    ctx.set_visuals(v);
}
