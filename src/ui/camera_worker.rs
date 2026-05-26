use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
    mpsc::{Sender, sync_channel},
};
use std::thread::{self, JoinHandle};

use image::DynamicImage;
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

pub struct CameraSession {
    pub stop_flag: Arc<AtomicBool>,
    pub handle: Option<JoinHandle<()>>,
}

impl CameraSession {
    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

pub fn start_camera(tx: Sender<DynamicImage>) -> Option<CameraSession> {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop_flag);
    let (init_tx, init_rx) = sync_channel::<bool>(1);

    let handle = thread::spawn(move || {
        let format = RequestedFormat::new::<RgbFormat>(
            RequestedFormatType::AbsoluteHighestFrameRate,
        );
        let mut camera = match Camera::new(CameraIndex::Index(0), format) {
            Ok(c) => c,
            Err(_) => {
                let _ = init_tx.send(false);
                return;
            }
        };
        if camera.open_stream().is_err() {
            let _ = init_tx.send(false);
            return;
        }
        let _ = init_tx.send(true);

        while !stop_clone.load(Ordering::Relaxed) {
            let frame = match camera.frame() {
                Ok(f) => f,
                Err(_) => break,
            };
            let decoded = match frame.decode_image::<RgbFormat>() {
                Ok(d) => d,
                Err(_) => continue,
            };
            let dyn_img = DynamicImage::ImageRgb8(decoded);
            if tx.send(dyn_img).is_err() {
                break;
            }
        }
        let _ = camera.stop_stream();
    });

    match init_rx.recv() {
        Ok(true) => Some(CameraSession {
            stop_flag,
            handle: Some(handle),
        }),
        _ => {
            let _ = handle.join();
            None
        }
    }
}
