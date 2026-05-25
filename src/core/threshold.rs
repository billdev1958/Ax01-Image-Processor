use image::{GrayImage, Luma};

pub fn threshold(input: &GrayImage, threshold: u8) -> GrayImage {
    let (width, height) = input.dimensions();
    let mut output = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel_val = input.get_pixel(x, y)[0];

            let out_val = if pixel_val >= threshold {255} else {0};

            output.put_pixel(x, y, Luma([out_val]));
        }

    }

    output
}
