use super::{Frame, ScreenCapture};
use anyhow::{Context, Result};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct WindowsCapturer {
    width: u32,
    height: u32,
    running: bool,
    capturer: scrap::Capturer,
}

impl WindowsCapturer {
    pub fn new() -> Result<()> {
        let display = scrap::Display::primary()
            .context("Failed to get primary display")?;

        let width = display.width() as u32;
        let height = display.height() as u32;

        let capturer = scrap::Capturer::new(display)
            .context("Failed to create capturer")?;

        Ok(Self {
            width,
            height,
            running: false,
            capturer,
        })
    }
}

impl ScreenCapture for WindowsCapturer {
    fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Windows screen capture: {}x{}", self.width, self.height);
        self.running = true;
        Ok(())
    }

    fn capture_frame(&mut self) -> Result<Frame> {
        if !self.running {
            anyhow::bail!("Capturer not started");
        }

        let frame = loop {
            match self.capturer.frame() {
                Ok(frame) => break frame,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Frame not ready, wait a bit
                    std::thread::sleep(std::time::Duration::from_millis(1));
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // scrap gives us BGRA, convert to RGBA
        let mut rgba_data = Vec::with_capacity(frame.len());
        for chunk in frame.chunks_exact(4) {
            rgba_data.push(chunk[2]); // R
            rgba_data.push(chunk[1]); // G
            rgba_data.push(chunk[0]); // B
            rgba_data.push(chunk[3]); // A
        }

        Ok(Frame {
            data: rgba_data,
            width: self.width,
            height: self.height,
            stride: (self.width * 4) as usize,
            timestamp,
        })
    }

    fn stop(&mut self) {
        tracing::info!("Stopping Windows screen capture");
        self.running = false;
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
