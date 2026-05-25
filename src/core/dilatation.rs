use image::{GrayImage, Luma};
use std::cmp;

pub fn apply_dilatation(input: &GrayImage, kernel_size: u32) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut output = GrayImage::new(width, height);

    let offset = (kernel_size / 2) as i32;

    for y in (offset as u32)..(height - offset as u32) {
        for x in (offset as u32)..(width - offset as u32) {
            let mut max_val = 0u8;

            for ky in -offset..=offset {
                for kx in -offset..=offset {
                    let px_x = (x as i32 + kx) as u32;
                    let px_y = (y as i32 + ky) as u32;

                    let pixel_val = input.get_pixel(px_x, px_y)[0];

                    max_val = cmp::max(max_val, pixel_val);
                }
            }
            output.put_pixel(x, y, Luma([max_val]));
        }
    }
    output
}
