use image::{RgbImage, GrayImage, Luma};

pub fn rgb_segmentation(input: &RgbImage, target_r: u8, target_g: u8, target_b: u8, tolerance: f32) -> GrayImage {
	
	let (width, height) = input.dimensions();
	let mut mask = GrayImage::new(width, height);

	let tr = target_r as f32;
	let tg = target_g as f32;
	let tb = target_b as f32;

	for y in 0..height {
		for x in 0..width {
			let pixel = input.get_pixel(x, y);

			let r = pixel[0] as f32;
			let g = pixel[1] as f32;
			let b = pixel[2] as f32;

			let dist_sq =	(r - tr) * (r -tr) +
								(g - tg) * (g - tg) +
								(b - tb) * (b - tb);

			let distance = dist_sq.sqrt();

			let out_val = if distance <= tolerance {255} else {0};

			mask.put_pixel(x, y, Luma([out_val]))
		}
	}
	mask
}
