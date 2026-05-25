use std::path::Path;
use egui::{ColorImage, Context, TextureHandle, TextureOptions};
use image::DynamicImage;

pub fn load_dynamic_image(path: &Path) -> Option<DynamicImage> {
    image::open(path).ok()
}

pub fn dynamic_to_texture(ctx: &Context, img: &DynamicImage, name: &str) -> TextureHandle {
    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let pixels = rgba.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    ctx.load_texture(name.to_string(), color_image, TextureOptions::LINEAR)
}
