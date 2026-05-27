use image::{GrayImage, Rgb, RgbImage};

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

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let cmax = r.max(g).max(b);
    let cmin = r.min(g).min(b);
    let delta = cmax - cmin;

    let mut h = 0.0;
    if delta > 0.0001 {
        if (cmax - r).abs() < 1e-6 {
            h = 60.0 * (((g - b) / delta) % 6.0);
        } else if (cmax - g).abs() < 1e-6 {
            h = 60.0 * ((b - r) / delta + 2.0);
        } else {
            h = 60.0 * ((r - g) / delta + 4.0);
        }
    }
    if h < 0.0 {
        h += 360.0;
    }

    let s = if cmax > 0.0001 { delta / cmax } else { 0.0 };
    (h, s, cmax)
}

pub fn is_shadow(current: &Rgb<u8>, reference: &Rgb<u8>) -> bool {
    let (h1, s1, v1) = rgb_to_hsv(current[0], current[1], current[2]);
    let (h2, s2, v2) = rgb_to_hsv(reference[0], reference[1], reference[2]);

    let mut dh = (h1 - h2).abs();
    if dh > 180.0 {
        dh = 360.0 - dh;
    }
    let ds = (s1 - s2).abs();
    let v_ratio = if v2 > 0.01 { v1 / v2 } else { 1.0 };

    dh < 15.0 && ds < 0.20 && v_ratio > 0.3 && v_ratio < 0.85
}
