use image::{GrayImage, Luma};

pub fn sobel(input: &GrayImage) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut output = GrayImage::new(width, height);

    let kernel_x = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];

    let kernel_y = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut gx = 0.0;
            let mut gy = 0.0;

            for ky in -1..=1 {
                for kx in -1..=1 {
                    let px_x = (x as i32 + kx) as u32;
                    let px_y = (y as i32 + ky) as u32;

                    let pixel_val = input.get_pixel(px_x, px_y)[0] as f32;

                    let idx_y = (ky + 1) as usize;
                    let idx_x = (kx + 1) as usize;

                    gx += pixel_val * (kernel_x[idx_y][idx_x] as f32);
                    gy += pixel_val * (kernel_y[idx_y][idx_x] as f32);
                }
            }

            let magnitude = (gx * gx + gy * gy).sqrt();

            let clamped_val = magnitude.clamp(0.0, 255.0) as u8;

            output.put_pixel(x, y, Luma([clamped_val]));
        }
    }

    output
}
