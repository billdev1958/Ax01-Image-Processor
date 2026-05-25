use image::{GrayImage, Luma};
use std::cmp;

pub fn apply_erosion(input: &GrayImage, kernel_size: u32) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut output = GrayImage::new(width, height);

    let offset = (kernel_size / 2) as i32;

    for y in (offset as u32)..(height - offset as u32) {
        for x in (offset as u32)..(width - offset as u32) {
            
            let mut min_val = 255u8;

            for ky in -offset..=offset{
                for kx in -offset..=offset{
                    let px_x = (x as i32 + kx) as u32;
                    let px_y = (y as i32 + ky) as u32;

                    let pixel_val = input.get_pixel(px_x, px_y)[0];

                    min_val = cmp::min(min_val, pixel_val);
                }
            }
            output.put_pixel(x, y, Luma([min_val]));
        }
    }
    output
}
