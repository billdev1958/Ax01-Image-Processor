use image::{GrayImage, Luma};

pub struct GaussianBackgroundModel {
    pub width: u32,
    pub height: u32,
    pub mean: Vec<f32>,
    pub variance: Vec<f32>,
    pub learning_rate: f32,
    pub threshold_k: f32,
}

impl GaussianBackgroundModel {
    pub fn new(width: u32, height: u32, learning_rate: f32, threshold_k: f32) -> Self {
        let total_pixels = (width * height) as usize;
        Self {
            width,
            height,
            mean: vec![0.0; total_pixels],
            variance: vec![10.0; total_pixels],
            learning_rate,
            threshold_k,
        }
    }

    pub fn process_frame(&mut self, current_frame: &GrayImage) -> GrayImage {
        let mut foreground_mask = GrayImage::new(self.width, self.height);
        let alpha = self.learning_rate;
        let one_minus_alpha = 1.0 - alpha;

        for (i, pixel) in current_frame.pixels().enumerate() {
            let pixel_val = pixel[0] as f32;

            let current_mean = self.mean[i];
            let current_variance = self.variance[i];

            let std_dev = current_variance.sqrt();

            let diff = (pixel_val - current_mean).abs();

            let is_foreground = diff > (self.threshold_k * std_dev);

            if is_foreground {
                let x = (i as u32) % self.width;
                let y = (i as u32) / self.width;
                foreground_mask.put_pixel(x, y, Luma([255]));
            } else {
                self.mean[i] = one_minus_alpha * current_mean + alpha * pixel_val;

                let val_diff = pixel_val - self.mean[i];

                self.variance[i] = one_minus_alpha * current_variance + alpha * (val_diff * val_diff);

                self.variance[i] = self.variance[i].max(1.0);
            }
        }
        foreground_mask
    }
}
