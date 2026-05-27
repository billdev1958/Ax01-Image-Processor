use egui::{Color32, CornerRadius, FontId, Margin, RichText, Stroke, StrokeKind, Vec2};
use crate::app::{App, BgViewMode, LastOp, View};
use crate::ui::theme::{JADE, NAV_ACTIVE, OBSIDIAN_0, OBSIDIAN_1, OBSIDIAN_2, TEXT};
use crate::ui::widgets::button::{jade_button, normal_button};
use crate::ui::widgets::image_pane::image_pane;

pub fn show(ctx: &egui::Context, app: &mut App) {
    let frame_sidebar = egui::Frame::new()
        .fill(OBSIDIAN_1)
        .inner_margin(Margin::same(12));

    let panel_resp = egui::SidePanel::left("sidebar")
        .exact_width(210.0)
        .resizable(false)
        .show_separator_line(false)
        .frame(frame_sidebar)
        .show(ctx, |ui| {
            ui.label(RichText::new("MorphoStudio").color(TEXT).size(15.0).strong());
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::new(36.0, 36.0), egui::Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(rect, CornerRadius::same(4u8), OBSIDIAN_2);
                painter.rect_stroke(
                    rect,
                    CornerRadius::same(4u8),
                    Stroke::new(1.0, Color32::from_gray(55)),
                    StrokeKind::Middle,
                );

                ui.add_space(4.0);
                ui.vertical(|ui| {
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("Project Alpha")
                            .color(TEXT)
                            .size(11.0)
                            .strong(),
                    );
                    ui.label(
                        RichText::new("1200 × 800px • 16-bit")
                            .size(9.0)
                            .color(Color32::from_gray(120)),
                    );
                });
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);

            let avail_h = ui.available_height();
            let footer_reserve = 22.0;
            let scroll_h = (avail_h - footer_reserve).max(0.0);
            egui::ScrollArea::vertical()
                .max_height(scroll_h)
                .auto_shrink([false, false])
                .show(ui, |ui| {
            nav_item_expandable(ui, app, "TOOLS");
            if app.tools_expanded {
                ui.add_space(1.0);
                if sub_nav_item(ui, "Erosion").clicked() {
                    apply_op(app, ui.ctx(), LastOp::Erosion);
                }
                if sub_nav_item(ui, "Dilation").clicked() {
                    apply_op(app, ui.ctx(), LastOp::Dilation);
                }
                ui.add_space(4.0);
                if sub_nav_item(ui, "Sobel").clicked() {
                    apply_op(app, ui.ctx(), LastOp::Sobel);
                }
                ui.add_space(4.0);
                if sub_nav_item(ui, "RGB Segmentation").clicked() {
                    apply_op(app, ui.ctx(), LastOp::RgbSegmentation);
                }
                if sub_nav_item(ui, "HSV Segmentation").clicked() {
                    apply_op(app, ui.ctx(), LastOp::HsvSegmentation);
                }

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("KERNEL SIZE")
                            .size(9.0)
                            .color(Color32::from_gray(140))
                            .strong(),
                    );
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    let (minus, plus) =
                        stepper(ui, "kernel", &app.kernel_size.to_string());
                    let mut changed = false;
                    if minus && app.kernel_size > 1 {
                        app.kernel_size -= 2;
                        changed = true;
                    }
                    if plus && app.kernel_size < 15 {
                        app.kernel_size += 2;
                        changed = true;
                    }
                    if changed
                        && matches!(
                            app.last_op,
                            Some(LastOp::Erosion) | Some(LastOp::Dilation)
                        )
                    {
                        if let Some(op) = app.last_op {
                            apply_op(app, ui.ctx(), op);
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("THRESHOLD")
                            .size(9.0)
                            .color(Color32::from_gray(140))
                            .strong(),
                    );
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.spacing_mut().slider_width = 100.0;
                    let resp = ui.add(egui::Slider::new(
                        &mut app.threshold_value,
                        0u8..=255,
                    ));
                    if resp.changed() {
                        recompute_display(app, ui.ctx());
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("RGB SEGMENTATION")
                            .size(9.0)
                            .color(Color32::from_gray(140))
                            .strong(),
                    );
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("R")
                            .size(9.0)
                            .color(Color32::from_gray(140)),
                    );
                    ui.spacing_mut().slider_width = 85.0;
                    let resp = ui.add(egui::Slider::new(
                        &mut app.rgb_target_r,
                        0u8..=255,
                    ));
                    if resp.changed()
                        && app.last_op == Some(LastOp::RgbSegmentation)
                    {
                        apply_op(app, ui.ctx(), LastOp::RgbSegmentation);
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("G")
                            .size(9.0)
                            .color(Color32::from_gray(140)),
                    );
                    ui.spacing_mut().slider_width = 85.0;
                    let resp = ui.add(egui::Slider::new(
                        &mut app.rgb_target_g,
                        0u8..=255,
                    ));
                    if resp.changed()
                        && app.last_op == Some(LastOp::RgbSegmentation)
                    {
                        apply_op(app, ui.ctx(), LastOp::RgbSegmentation);
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("B")
                            .size(9.0)
                            .color(Color32::from_gray(140)),
                    );
                    ui.spacing_mut().slider_width = 85.0;
                    let resp = ui.add(egui::Slider::new(
                        &mut app.rgb_target_b,
                        0u8..=255,
                    ));
                    if resp.changed()
                        && app.last_op == Some(LastOp::RgbSegmentation)
                    {
                        apply_op(app, ui.ctx(), LastOp::RgbSegmentation);
                    }
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("tol")
                            .size(9.0)
                            .color(Color32::from_gray(140)),
                    );
                    let (minus, plus) =
                        stepper(ui, "rgb_tol", &(app.rgb_tolerance as i32).to_string());
                    let mut changed = false;
                    if minus && app.rgb_tolerance > 0.0 {
                        app.rgb_tolerance = (app.rgb_tolerance - 5.0).max(0.0);
                        changed = true;
                    }
                    if plus && app.rgb_tolerance < 250.0 {
                        app.rgb_tolerance = (app.rgb_tolerance + 5.0).min(250.0);
                        changed = true;
                    }
                    if changed
                        && app.last_op == Some(LastOp::RgbSegmentation)
                    {
                        apply_op(app, ui.ctx(), LastOp::RgbSegmentation);
                    }
                });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("HSV SEGMENTATION")
                            .size(9.0)
                            .color(Color32::from_gray(140))
                            .strong(),
                    );
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("H")
                            .size(9.0)
                            .color(Color32::from_gray(140)),
                    );
                    ui.spacing_mut().slider_width = 85.0;
                    let resp = ui.add(egui::Slider::new(
                        &mut app.hsv_target_hue,
                        0.0f32..=360.0,
                    ));
                    if resp.changed()
                        && app.last_op == Some(LastOp::HsvSegmentation)
                    {
                        apply_op(app, ui.ctx(), LastOp::HsvSegmentation);
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("tol")
                            .size(9.0)
                            .color(Color32::from_gray(140)),
                    );
                    let (minus, plus) = stepper(
                        ui,
                        "hsv_tol",
                        &(app.hsv_tolerance as i32).to_string(),
                    );
                    let mut changed = false;
                    if minus && app.hsv_tolerance > 0.0 {
                        app.hsv_tolerance = (app.hsv_tolerance - 5.0).max(0.0);
                        changed = true;
                    }
                    if plus && app.hsv_tolerance < 180.0 {
                        app.hsv_tolerance = (app.hsv_tolerance + 5.0).min(180.0);
                        changed = true;
                    }
                    if changed
                        && app.last_op == Some(LastOp::HsvSegmentation)
                    {
                        apply_op(app, ui.ctx(), LastOp::HsvSegmentation);
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    if normal_button(ui, "Clear filters").clicked() {
                        app.processed_data = None;
                        app.processed_image = None;
                        app.threshold_value = 0;
                        app.last_op = None;
                        app.processed_zoom = 1.0;
                        app.processed_pan = Vec2::ZERO;
                    }
                });
                ui.add_space(1.0);
            }
            ui.add_space(2.0);
            nav_item(ui, app, View::History, "BG SUBTRACT");

                });

            ui.add_space(4.0);
            ui.label(
                RichText::new("v0.1.0")
                    .color(Color32::from_gray(70))
                    .size(10.0),
            );
        });

    let r = panel_resp.response.rect.shrink(0.25);
    let painter = ctx.layer_painter(egui::LayerId::new(
        egui::Order::Foreground,
        egui::Id::new("sidebar_border"),
    ));
    painter.rect_stroke(
        r,
        CornerRadius::ZERO,
        Stroke::new(0.5, JADE),
        StrokeKind::Middle,
    );

    let toolbar_frame = egui::Frame::new()
        .fill(OBSIDIAN_1)
        .inner_margin(Margin::symmetric(12, 6));

    egui::TopBottomPanel::top("toolbar")
        .frame(toolbar_frame)
        .show_separator_line(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if jade_button(ui, "Import Image").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(
                            "Image",
                            &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tif", "tiff"],
                        )
                        .pick_file()
                    {
                        if let Some(dyn_img) =
                            crate::ui::image_loader::load_dynamic_image(&path)
                        {
                            let name = path
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("image");
                            app.original_image =
                                Some(crate::ui::image_loader::dynamic_to_texture(
                                    ui.ctx(),
                                    &dyn_img,
                                    name,
                                ));
                            app.original_data = Some(dyn_img);
                            app.processed_image = None;
                            app.processed_data = None;
                            app.threshold_value = 0;
                            app.last_op = None;
                            app.processed_zoom = 1.0;
                            app.processed_pan = Vec2::ZERO;
                        }
                    }
                }
                ui.add_space(8.0);
                if normal_button(ui, "Export Image").clicked() {
                    if let Some(img) = build_export_image(app) {
                        if let Some(mut path) = rfd::FileDialog::new()
                            .set_file_name("processed.png")
                            .add_filter("PNG", &["png"])
                            .add_filter("JPEG", &["jpg", "jpeg"])
                            .add_filter("BMP", &["bmp"])
                            .add_filter("TIFF", &["tif", "tiff"])
                            .save_file()
                        {
                            if path.extension().is_none() {
                                path.set_extension("png");
                            }
                            let _ = img.save(&path);
                        }
                    }
                }
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        if normal_button(ui, "Clear").clicked() {
                            app.original_image = None;
                            app.original_data = None;
                            app.processed_image = None;
                            app.processed_data = None;
                            app.threshold_value = 0;
                            app.kernel_size = 3;
                            app.rgb_target_r = 0;
                            app.rgb_target_g = 0;
                            app.rgb_target_b = 0;
                            app.rgb_tolerance = 30.0;
                            app.hsv_target_hue = 0.0;
                            app.hsv_tolerance = 15.0;
                            app.last_op = None;
                            app.original_zoom = 1.0;
                            app.processed_zoom = 1.0;
                            app.original_pan = Vec2::ZERO;
                            app.processed_pan = Vec2::ZERO;
                        }
                    },
                );
            });
        });

    let frame_central = egui::Frame::new()
        .fill(OBSIDIAN_0)
        .inner_margin(Margin::same(8));

    egui::CentralPanel::default()
        .frame(frame_central)
        .show(ctx, |ui| {
            if app.current_view == View::History {
                bg_subtract_central(ui, app);
            } else {
                let avail = ui.available_size();
                let gap = 8.0;
                let pane_size = Vec2::new((avail.x - gap) / 2.0, avail.y);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    image_pane(
                        ui,
                        pane_size,
                        "Original Image",
                        app.original_image.as_ref(),
                        &mut app.original_zoom,
                        &mut app.original_pan,
                    );
                    ui.add_space(gap);
                    image_pane(
                        ui,
                        pane_size,
                        "Processed Image",
                        app.processed_image.as_ref(),
                        &mut app.processed_zoom,
                        &mut app.processed_pan,
                    );
                });
            }
        });
}

fn bg_subtract_central(ui: &mut egui::Ui, app: &mut App) {
    if let Some(rx) = app.bg_frame_rx.as_ref() {
        let mut latest: Option<image::DynamicImage> = None;
        while let Ok(frame) = rx.try_recv() {
            latest = Some(frame);
        }
        if let Some(frame) = latest {
            let gray = frame.to_luma8();
            let rgb = frame.to_rgb8();
            let (w, h) = gray.dimensions();
            let total = (w * h) as usize;

            if app.bg_model.is_none() {
                let mut model =
                    crate::core::gaussian_background::GaussianBackgroundModel::new(
                        w, h, 0.01, 2.5,
                    );
                for (i, p) in gray.pixels().enumerate() {
                    model.mean[i] = p[0] as f32;
                    model.variance[i] = 200.0;
                }
                app.bg_model = Some(model);
                app.bg_clean_frame = Some(rgb.clone());
                app.motion_history = Some(vec![0.0; total]);
            }

            let raw_mask = app.bg_model.as_mut().unwrap().process_frame(&gray);
            let eroded = crate::core::erotion::apply_erosion(&raw_mask, 3);
            let opened = crate::core::dilatation::apply_dilatation(&eroded, 3);
            let dilated_more = crate::core::dilatation::apply_dilatation(&opened, 5);
            let cleaned = crate::core::erotion::apply_erosion(&dilated_more, 5);

            app.bg_camera_image = Some(crate::ui::image_loader::dynamic_to_texture(
                ui.ctx(),
                &frame,
                "bg_cam",
            ));

            let bg_clean = app.bg_clean_frame.as_ref().unwrap();
            let history = app.motion_history.as_mut().unwrap();

            let mut classification = vec![0u8; total];
            for y in 0..h {
                for x in 0..w {
                    let i = (y * w + x) as usize;
                    if cleaned.get_pixel(x, y)[0] > 127 {
                        let curr = rgb.get_pixel(x, y);
                        let ref_pix = bg_clean.get_pixel(x, y);
                        classification[i] = if crate::ui::effects::is_shadow(curr, ref_pix) {
                            2
                        } else {
                            1
                        };
                    }
                }
            }

            for v in history.iter_mut() {
                *v = (*v - 5.0).max(0.0);
            }
            for i in 0..total {
                if classification[i] == 1 {
                    history[i] = 255.0;
                }
            }

            let right_pane_dyn = match app.bg_view_mode {
                BgViewMode::Mask => {
                    let mut output = image::GrayImage::new(w, h);
                    for y in 0..h {
                        for x in 0..w {
                            let i = (y * w + x) as usize;
                            let val = match classification[i] {
                                1 => 255,
                                2 => 128,
                                _ => history[i] as u8,
                            };
                            output.put_pixel(x, y, image::Luma([val]));
                        }
                    }
                    image::DynamicImage::ImageLuma8(output)
                }
                BgViewMode::Predator => {
                    let mut real_fg = image::GrayImage::new(w, h);
                    for y in 0..h {
                        for x in 0..w {
                            let i = (y * w + x) as usize;
                            if classification[i] == 1 {
                                real_fg.put_pixel(x, y, image::Luma([255]));
                            }
                        }
                    }
                    let composite = crate::ui::effects::predator_composite(
                        &rgb, bg_clean, &real_fg, 15, 15,
                    );
                    image::DynamicImage::ImageRgb8(composite)
                }
            };

            app.bg_mask_image = Some(crate::ui::image_loader::dynamic_to_texture(
                ui.ctx(),
                &right_pane_dyn,
                "bg_right",
            ));
        }
    }

    if app.bg_session.is_some() {
        ui.ctx().request_repaint();
    }

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let running = app.bg_session.is_some();
        let btn = if running {
            normal_button(ui, "Stop Camera")
        } else {
            jade_button(ui, "Start Camera")
        };
        if btn.clicked() {
            if running {
                if let Some(mut s) = app.bg_session.take() {
                    s.stop();
                }
                app.bg_frame_rx = None;
                app.bg_model = None;
                app.bg_clean_frame = None;
                app.motion_history = None;
            } else {
                let (tx, rx) = std::sync::mpsc::channel();
                if let Some(session) = crate::ui::camera_worker::start_camera(tx) {
                    app.bg_session = Some(session);
                    app.bg_frame_rx = Some(rx);
                    app.bg_camera_image = None;
                    app.bg_mask_image = None;
                    app.bg_model = None;
                    app.bg_clean_frame = None;
                    app.motion_history = None;
                }
            }
        }
        if running {
            ui.add_space(10.0);
            ui.label(
                RichText::new("● Camera running")
                    .color(JADE)
                    .size(11.0),
            );
        }
        ui.add_space(20.0);
        ui.label(RichText::new("Mode:").size(11.0).color(TEXT));
        ui.selectable_value(&mut app.bg_view_mode, BgViewMode::Mask, "Mask");
        ui.selectable_value(&mut app.bg_view_mode, BgViewMode::Predator, "Predator");
    });
    ui.add_space(8.0);

    let avail = ui.available_size();
    let gap = 8.0;
    let pane_size = Vec2::new((avail.x - gap) / 2.0, avail.y);

    let mut zoom_left = 1.0_f32;
    let mut pan_left = Vec2::ZERO;
    let mut zoom_right = 1.0_f32;
    let mut pan_right = Vec2::ZERO;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        image_pane(
            ui,
            pane_size,
            "Camera",
            app.bg_camera_image.as_ref(),
            &mut zoom_left,
            &mut pan_left,
        );
        ui.add_space(gap);
        let right_label = match app.bg_view_mode {
            BgViewMode::Mask => "Foreground Mask",
            BgViewMode::Predator => "Predator",
        };
        image_pane(
            ui,
            pane_size,
            right_label,
            app.bg_mask_image.as_ref(),
            &mut zoom_right,
            &mut pan_right,
        );
    });
}

fn build_export_image(app: &App) -> Option<image::DynamicImage> {
    if app.threshold_value > 0 {
        let source = app
            .processed_data
            .as_ref()
            .or(app.original_data.as_ref())?;
        let gray = source.to_luma8();
        let result = crate::core::threshold::threshold(&gray, app.threshold_value);
        Some(image::DynamicImage::ImageLuma8(result))
    } else {
        app.processed_data.clone()
    }
}

fn apply_op(app: &mut App, ctx: &egui::Context, op: LastOp) {
    let kernel = app.kernel_size;
    let r = app.rgb_target_r;
    let g = app.rgb_target_g;
    let b = app.rgb_target_b;
    let tol = app.rgb_tolerance;
    let hue = app.hsv_target_hue;
    let hsv_tol = app.hsv_tolerance;
    let Some(orig) = app.original_data.as_ref() else {
        return;
    };
    let result = match op {
        LastOp::Erosion => {
            let gray = orig.to_luma8();
            crate::core::erotion::apply_erosion(&gray, kernel)
        }
        LastOp::Dilation => {
            let gray = orig.to_luma8();
            crate::core::dilatation::apply_dilatation(&gray, kernel)
        }
        LastOp::Sobel => {
            let gray = orig.to_luma8();
            crate::core::sobel::sobel(&gray)
        }
        LastOp::RgbSegmentation => {
            let rgb = orig.to_rgb8();
            crate::core::rgb_segmentation::rgb_segmentation(&rgb, r, g, b, tol)
        }
        LastOp::HsvSegmentation => {
            let rgb = orig.to_rgb8();
            crate::core::hsv_segmentation::hsv_segmentation(&rgb, hue, hsv_tol)
        }
    };
    app.processed_data = Some(image::DynamicImage::ImageLuma8(result));
    app.processed_zoom = 1.0;
    app.processed_pan = Vec2::ZERO;
    app.last_op = Some(op);
    recompute_display(app, ctx);
}

fn recompute_display(app: &mut App, ctx: &egui::Context) {
    let threshold = app.threshold_value;
    let new_image = if threshold > 0 {
        let source = app
            .processed_data
            .as_ref()
            .or(app.original_data.as_ref());
        source.map(|d| {
            let gray = d.to_luma8();
            let result = crate::core::threshold::threshold(&gray, threshold);
            let dyn_result = image::DynamicImage::ImageLuma8(result);
            crate::ui::image_loader::dynamic_to_texture(ctx, &dyn_result, "processed")
        })
    } else {
        app.processed_data
            .as_ref()
            .map(|d| crate::ui::image_loader::dynamic_to_texture(ctx, d, "processed"))
    };
    app.processed_image = new_image;
}

fn stepper(ui: &mut egui::Ui, id_seed: &str, display: &str) -> (bool, bool) {
    let row_h = 22.0;
    let btn_w = 26.0;
    let display_w = 56.0;
    let total_w = btn_w + display_w + btn_w;

    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(total_w, row_h), egui::Sense::hover());

    let minus_rect =
        egui::Rect::from_min_size(rect.min, Vec2::new(btn_w, row_h));
    let display_rect = egui::Rect::from_min_size(
        egui::pos2(minus_rect.right(), rect.top()),
        Vec2::new(display_w, row_h),
    );
    let plus_rect = egui::Rect::from_min_size(
        egui::pos2(display_rect.right(), rect.top()),
        Vec2::new(btn_w, row_h),
    );

    let id_root = ui.id().with("stepper").with(id_seed);
    let minus_resp =
        ui.interact(minus_rect, id_root.with("minus"), egui::Sense::click());
    let plus_resp =
        ui.interact(plus_rect, id_root.with("plus"), egui::Sense::click());

    let painter = ui.painter();
    let border = Stroke::new(0.5, Color32::from_gray(55));

    let minus_bg = if minus_resp.hovered() {
        Color32::from_gray(45)
    } else {
        OBSIDIAN_2
    };
    painter.rect_filled(minus_rect, CornerRadius::same(3u8), minus_bg);
    painter.rect_stroke(minus_rect, CornerRadius::same(3u8), border, StrokeKind::Inside);
    painter.text(
        minus_rect.center(),
        egui::Align2::CENTER_CENTER,
        "−",
        FontId::proportional(13.0),
        TEXT,
    );

    painter.text(
        display_rect.center(),
        egui::Align2::CENTER_CENTER,
        display,
        FontId::proportional(11.0),
        TEXT,
    );

    let plus_bg = if plus_resp.hovered() {
        Color32::from_gray(45)
    } else {
        OBSIDIAN_2
    };
    painter.rect_filled(plus_rect, CornerRadius::same(3u8), plus_bg);
    painter.rect_stroke(plus_rect, CornerRadius::same(3u8), border, StrokeKind::Inside);
    painter.text(
        plus_rect.center(),
        egui::Align2::CENTER_CENTER,
        "+",
        FontId::proportional(14.0),
        TEXT,
    );

    (minus_resp.clicked(), plus_resp.clicked())
}

fn nav_item_expandable(ui: &mut egui::Ui, app: &mut App, label: &str) {
    let width = ui.available_width();
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(width, 30.0), egui::Sense::click());

    let active = app.current_view == View::Tools;
    let fill = if active {
        NAV_ACTIVE
    } else if resp.hovered() {
        Color32::from_gray(34)
    } else {
        Color32::TRANSPARENT
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4u8), fill);

    let text_pos = rect.left_center() + Vec2::new(12.0, 0.0);
    painter.text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.0),
        TEXT,
    );

    let cx = rect.right() - 16.0;
    let cy = rect.center().y;
    let stroke = Stroke::new(1.2, Color32::from_gray(100));
    if app.tools_expanded {
        painter.line_segment(
            [egui::pos2(cx - 4.0, cy - 2.0), egui::pos2(cx, cy + 2.0)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(cx + 4.0, cy - 2.0), egui::pos2(cx, cy + 2.0)],
            stroke,
        );
    } else {
        painter.line_segment(
            [egui::pos2(cx - 2.0, cy - 4.0), egui::pos2(cx + 2.0, cy)],
            stroke,
        );
        painter.line_segment(
            [egui::pos2(cx - 2.0, cy + 4.0), egui::pos2(cx + 2.0, cy)],
            stroke,
        );
    }

    if resp.clicked() {
        app.current_view = View::Tools;
        app.tools_expanded = !app.tools_expanded;
    }
}

fn sub_nav_item(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let width = ui.available_width();
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(width, 26.0), egui::Sense::click());

    let fill = if resp.hovered() {
        Color32::from_gray(32)
    } else {
        Color32::TRANSPARENT
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4u8), fill);

    painter.circle_filled(
        egui::pos2(rect.left() + 22.0, rect.center().y),
        2.0,
        Color32::from_gray(60),
    );

    painter.text(
        rect.left_center() + Vec2::new(32.0, 0.0),
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(10.0),
        Color32::from_gray(160),
    );

    resp
}

fn nav_item(ui: &mut egui::Ui, app: &mut App, view: View, label: &str) {
    let width = ui.available_width();
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(width, 30.0), egui::Sense::click());

    let active = app.current_view == view;
    let fill = if active {
        NAV_ACTIVE
    } else if resp.hovered() {
        Color32::from_gray(34)
    } else {
        Color32::TRANSPARENT
    };

    let painter = ui.painter();
    painter.rect_filled(rect, CornerRadius::same(4u8), fill);

    let text_pos = rect.left_center() + Vec2::new(12.0, 0.0);
    painter.text(
        text_pos,
        egui::Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.0),
        TEXT,
    );

    if resp.clicked() {
        app.current_view = view;
    }
}

