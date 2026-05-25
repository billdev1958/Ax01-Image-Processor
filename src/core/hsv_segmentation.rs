use image::{GrayImage, Luma, RgbImage};

pub fn hsv_segmentation(input: &RgbImage, target_hue: f32, tolerance: f32) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut mask = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = input.get_pixel(x, y);

            let r = (pixel[0] as f32) / 255.0;
            let g = (pixel[1] as f32) / 255.0;
            let b = (pixel[2] as f32) / 255.0;

            let cmax = r.max(g).max(b);
            let cmin = r.min(g).min(b);
            let delta = cmax - cmin;

            let mut hue = 0.0;
            if delta > 0.0001 {
                if cmax == r {
                    hue = 60.0 * (((g - b) / delta) % 6.0)
                } else if cmax == g {
                    hue = 60.0 * (((b - r) / delta) % 2.0)
                } else if cmax == b {
                    hue = 60.0 * (((r - g) / delta) % 6.0)
                }
            }

            if hue < 0.0 {
                hue += 360.0
            }

            let mut diff = (hue - target_hue).abs();
            if diff > 180.0 {
                diff = 360.0 - diff;
            }

            let out_val = if diff <= tolerance { 255 } else { 0 };

            mask.put_pixel(x, y, Luma([out_val]));
        }
    }

    mask
}
