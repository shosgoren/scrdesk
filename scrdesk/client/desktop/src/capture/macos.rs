use super::{Frame, ScreenCapture};
use anyhow::{Context, Result};
use core_graphics::display::CGDisplay;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MacOSCapturer {
    display_id: u32,
    width: u32,
    height: u32,
    running: bool,
}

impl MacOSCapturer {
    pub fn new() -> Result<Self> {
        let display_id = CGDisplay::main().id;
        let bounds = CGDisplay::new(display_id).bounds();

        Ok(Self {
            display_id,
            width: bounds.size.width as u32,
            height: bounds.size.height as u32,
            running: false,
        })
    }
}

impl ScreenCapture for MacOSCapturer {
    fn start(&mut self) -> Result<()> {
        tracing::info!("Starting macOS screen capture: {}x{}", self.width, self.height);
        self.running = true;
        Ok(())
    }

    fn capture_frame(&mut self) -> Result<Frame> {
        if !self.running {
            anyhow::bail!("Capturer not started");
        }

        let display = CGDisplay::new(self.display_id);
        let image = display
            .image()
            .context("Failed to capture screen image")?;

        let width = image.width() as u32;
        let height = image.height() as u32;
        let bytes_per_row = image.bytes_per_row();
        let data = image.data();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // Convert CGImage data to RGBA
        let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
        let bytes_per_pixel = 4;

        for y in 0..height {
            for x in 0..width {
                let offset = (y as usize * bytes_per_row) + (x as usize * bytes_per_pixel);
                if offset + 3 < data.len() as usize {
                    rgba_data.push(data[offset + 2]); // R
                    rgba_data.push(data[offset + 1]); // G
                    rgba_data.push(data[offset]);     // B
                    rgba_data.push(data[offset + 3]); // A
                }
            }
        }

        Ok(Frame {
            data: rgba_data,
            width,
            height,
            stride: (width * 4) as usize,
            timestamp,
        })
    }

    fn stop(&mut self) {
        tracing::info!("Stopping macOS screen capture");
        self.running = false;
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
