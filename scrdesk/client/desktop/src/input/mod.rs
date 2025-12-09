use anyhow::Result;
use crate::protocol::{KeyModifiers, MouseButton};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

pub trait InputSimulator: Send {
    fn simulate_mouse_move(&self, x: i32, y: i32) -> Result<()>;
    fn simulate_mouse_button(&self, button: MouseButton, pressed: bool) -> Result<()>;
    fn simulate_mouse_scroll(&self, delta_x: i32, delta_y: i32) -> Result<()>;
    fn simulate_key(&self, key: &str, pressed: bool, modifiers: KeyModifiers) -> Result<()>;
}

pub fn create_simulator() -> Result<Box<dyn InputSimulator>> {
    #[cfg(target_os = "macos")]
    {
        Ok(Box::new(macos::MacOSSimulator::new()?))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(Box::new(windows::WindowsSimulator::new()?))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(Box::new(linux::LinuxSimulator::new()?))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        anyhow::bail!("Unsupported platform for input simulation")
    }
}
