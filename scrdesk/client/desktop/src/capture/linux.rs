use super::{Frame, ScreenCapture};
use anyhow::Result;

pub struct LinuxCapturer {
    width: u32,
    height: u32,
    running: bool,
}

impl LinuxCapturer {
    pub fn new() -> Result<Self> {
        // TODO: Implement X11/Wayland capture
        Ok(Self {
            width: 1920,
            height: 1080,
            running: false,
        })
    }
}

impl ScreenCapture for LinuxCapturer {
    fn start(&mut self) -> Result<()> {
        tracing::info!("Starting Linux screen capture: {}x{}", self.width, self.height);
        self.running = true;
        Ok(())
    }

    fn capture_frame(&mut self) -> Result<Frame> {
        anyhow::bail!("Linux screen capture not implemented yet")
    }

    fn stop(&mut self) {
        tracing::info!("Stopping Linux screen capture");
        self.running = false;
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
