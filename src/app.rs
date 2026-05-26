use crate::ui::screens::showcase;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum View {
    Tools,
    #[default]
    Library,
    History,
    Layers,
    Export,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LastOp {
    Erosion,
    Dilation,
    Sobel,
    RgbSegmentation,
    HsvSegmentation,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BgViewMode {
    Mask,
    Predator,
}

pub struct App {
    pub current_view: View,
    pub tools_expanded: bool,
    pub original_image: Option<egui::TextureHandle>,
    pub original_data: Option<image::DynamicImage>,
    pub processed_image: Option<egui::TextureHandle>,
    pub processed_data: Option<image::DynamicImage>,
    pub original_zoom: f32,
    pub processed_zoom: f32,
    pub original_pan: egui::Vec2,
    pub processed_pan: egui::Vec2,
    pub threshold_value: u8,
    pub kernel_size: u32,
    pub rgb_target_r: u8,
    pub rgb_target_g: u8,
    pub rgb_target_b: u8,
    pub rgb_tolerance: f32,
    pub hsv_target_hue: f32,
    pub hsv_tolerance: f32,
    pub last_op: Option<LastOp>,
    pub bg_session: Option<crate::ui::camera_worker::CameraSession>,
    pub bg_frame_rx: Option<std::sync::mpsc::Receiver<image::DynamicImage>>,
    pub bg_model: Option<crate::core::gaussian_background::GaussianBackgroundModel>,
    pub bg_camera_image: Option<egui::TextureHandle>,
    pub bg_mask_image: Option<egui::TextureHandle>,
    pub bg_clean_frame: Option<image::RgbImage>,
    pub bg_view_mode: BgViewMode,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::ui::theme::apply_theme(&cc.egui_ctx);
        Self {
            current_view: View::default(),
            tools_expanded: false,
            original_image: None,
            original_data: None,
            processed_image: None,
            processed_data: None,
            original_zoom: 1.0,
            processed_zoom: 1.0,
            original_pan: egui::Vec2::ZERO,
            processed_pan: egui::Vec2::ZERO,
            threshold_value: 0,
            kernel_size: 3,
            rgb_target_r: 0,
            rgb_target_g: 0,
            rgb_target_b: 0,
            rgb_tolerance: 30.0,
            hsv_target_hue: 0.0,
            hsv_tolerance: 15.0,
            last_op: None,
            bg_session: None,
            bg_frame_rx: None,
            bg_model: None,
            bg_camera_image: None,
            bg_mask_image: None,
            bg_clean_frame: None,
            bg_view_mode: BgViewMode::Mask,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        showcase::show(ctx, self);
    }
}
