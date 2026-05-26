use image::{GrayImage, RgbImage};

pub fn predator_composite(
    current: &RgbImage,
    bg_clean: &RgbImage,
    mask: &GrayImage,
    offset_x: i32,
    offset_y: i32,
) -> RgbImage {
    let (w, h) = current.dimensions();
    let mut out = RgbImage::new(w, h);
    let wi = w as i32;
    let hi = h as i32;

    for y in 0..h {
        for x in 0..w {
            if mask.get_pixel(x, y)[0] > 127 {
                let sx = (x as i32 + offset_x).clamp(0, wi - 1) as u32;
                let sy = (y as i32 + offset_y).clamp(0, hi - 1) as u32;
                out.put_pixel(x, y, *bg_clean.get_pixel(sx, sy));
            } else {
                out.put_pixel(x, y, *current.get_pixel(x, y));
            }
        }
    }

    out
}
