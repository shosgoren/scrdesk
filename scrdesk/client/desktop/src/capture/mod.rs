use anyhow::Result;
use image::{ImageBuffer, Rgba};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub timestamp: u64,
}

pub trait ScreenCapture: Send {
    fn start(&mut self) -> Result<()>;
    fn capture_frame(&mut self) -> Result<Frame>;
    fn stop(&mut self);
    fn get_dimensions(&self) -> (u32, u32);
}

pub fn create_capturer() -> Result<Box<dyn ScreenCapture>> {
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(macos::MacOSCapturer::new()?))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(windows::WindowsCapturer::new()?))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(linux::LinuxCapturer::new()?))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        anyhow::bail!("Unsupported platform for screen capture")
    }
}
