use super::InputSimulator;
use crate::protocol::{KeyModifiers, MouseButton};
use anyhow::Result;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEEVENTF_XDOWN,
    MOUSEEVENTF_XUP, MOUSEINPUT, VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN,
    VK_END, VK_ESCAPE, VK_HOME, VK_LEFT, VK_MENU, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT,
    VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
};

// XBUTTON constants for extended mouse buttons
const XBUTTON1: u16 = 0x0001;
const XBUTTON2: u16 = 0x0002;

pub struct WindowsSimulator;

impl WindowsSimulator {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    fn send_mouse_input(&self, flags: u32, data: u32, dx: i32, dy: i32) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx,
                        dy,
                        mouseData: data,
                        dwFlags: MOUSEEVENTF_ABSOLUTE | flags.into(),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            let result = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            if result == 0 {
                anyhow::bail!("Failed to send mouse input");
            }
        }

        Ok(())
    }

    fn send_keyboard_input(&self, vk: VIRTUAL_KEY, flags: KEYBD_EVENT_FLAGS) -> Result<()> {
        unsafe {
            let input = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: vk,
                        wScan: 0,
                        dwFlags: flags,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };

            let result = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
            if result == 0 {
                anyhow::bail!("Failed to send keyboard input");
            }
        }

        Ok(())
    }

    fn map_mouse_button(&self, button: MouseButton) -> (u32, u32) {
        // Returns (down_flag, up_flag)
        match button {
            MouseButton::Left => (MOUSEEVENTF_LEFTDOWN.0, MOUSEEVENTF_LEFTUP.0),
            MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN.0, MOUSEEVENTF_RIGHTUP.0),
            MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN.0, MOUSEEVENTF_MIDDLEUP.0),
            MouseButton::Back => (MOUSEEVENTF_XDOWN.0, MOUSEEVENTF_XUP.0),
            MouseButton::Forward => (MOUSEEVENTF_XDOWN.0, MOUSEEVENTF_XUP.0),
        }
    }

    fn get_mouse_data(&self, button: MouseButton) -> u32 {
        match button {
            MouseButton::Back => XBUTTON1 as u32,
            MouseButton::Forward => XBUTTON2 as u32,
            _ => 0,
        }
    }
}

impl InputSimulator for WindowsSimulator {
    fn simulate_mouse_move(&self, x: i32, y: i32) -> Result<()> {
        // Convert screen coordinates to normalized absolute coordinates (0-65535)
        // Get screen dimensions
        let screen_width = unsafe { windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CXSCREEN) };
        let screen_height = unsafe { windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics(windows::Win32::UI::WindowsAndMessaging::SM_CYSCREEN) };

        let normalized_x = (x as f64 / screen_width as f64 * 65535.0) as i32;
        let normalized_y = (y as f64 / screen_height as f64 * 65535.0) as i32;

        self.send_mouse_input(MOUSEEVENTF_MOVE.0, 0, normalized_x, normalized_y)
    }

    fn simulate_mouse_button(&self, button: MouseButton, pressed: bool) -> Result<()> {
        let (down_flag, up_flag) = self.map_mouse_button(button);
        let flag = if pressed { down_flag } else { up_flag };
        let data = self.get_mouse_data(button);

        self.send_mouse_input(flag, data, 0, 0)
    }

    fn simulate_mouse_scroll(&self, _delta_x: i32, delta_y: i32) -> Result<()> {
        // Windows scroll is in WHEEL_DELTA units (120 units = 1 notch)
        // Positive delta_y = scroll up
        let wheel_delta = (delta_y * 120) as u32;

        self.send_mouse_input(MOUSEEVENTF_WHEEL.0, wheel_delta, 0, 0)
    }

    fn simulate_key(&self, key: &str, pressed: bool, modifiers: KeyModifiers) -> Result<()> {
        let vk = map_key_to_vk(key)?;

        // Press modifiers first if key is being pressed
        if pressed {
            if modifiers.shift {
                self.send_keyboard_input(VK_SHIFT, KEYBD_EVENT_FLAGS(0))?;
            }
            if modifiers.ctrl {
                self.send_keyboard_input(VK_CONTROL, KEYBD_EVENT_FLAGS(0))?;
            }
            if modifiers.alt {
                self.send_keyboard_input(VK_MENU, KEYBD_EVENT_FLAGS(0))?;
            }
            if modifiers.meta {
                // Windows key
                self.send_keyboard_input(VIRTUAL_KEY(0x5B), KEYBD_EVENT_FLAGS(0))?;
            }
        }

        // Send the actual key
        let flags = if pressed {
            KEYBD_EVENT_FLAGS(0)
        } else {
            KEYEVENTF_KEYUP
        };
        self.send_keyboard_input(vk, flags)?;

        // Release modifiers if key is being released
        if !pressed {
            if modifiers.shift {
                self.send_keyboard_input(VK_SHIFT, KEYEVENTF_KEYUP)?;
            }
            if modifiers.ctrl {
                self.send_keyboard_input(VK_CONTROL, KEYEVENTF_KEYUP)?;
            }
            if modifiers.alt {
                self.send_keyboard_input(VK_MENU, KEYEVENTF_KEYUP)?;
            }
            if modifiers.meta {
                self.send_keyboard_input(VIRTUAL_KEY(0x5B), KEYEVENTF_KEYUP)?;
            }
        }

        Ok(())
    }
}

fn map_key_to_vk(key: &str) -> Result<VIRTUAL_KEY> {
    let vk = match key.to_lowercase().as_str() {
        // Letters (A-Z are 0x41-0x5A)
        "a" => VIRTUAL_KEY(0x41),
        "b" => VIRTUAL_KEY(0x42),
        "c" => VIRTUAL_KEY(0x43),
        "d" => VIRTUAL_KEY(0x44),
        "e" => VIRTUAL_KEY(0x45),
        "f" => VIRTUAL_KEY(0x46),
        "g" => VIRTUAL_KEY(0x47),
        "h" => VIRTUAL_KEY(0x48),
        "i" => VIRTUAL_KEY(0x49),
        "j" => VIRTUAL_KEY(0x4A),
        "k" => VIRTUAL_KEY(0x4B),
        "l" => VIRTUAL_KEY(0x4C),
        "m" => VIRTUAL_KEY(0x4D),
        "n" => VIRTUAL_KEY(0x4E),
        "o" => VIRTUAL_KEY(0x4F),
        "p" => VIRTUAL_KEY(0x50),
        "q" => VIRTUAL_KEY(0x51),
        "r" => VIRTUAL_KEY(0x52),
        "s" => VIRTUAL_KEY(0x53),
        "t" => VIRTUAL_KEY(0x54),
        "u" => VIRTUAL_KEY(0x55),
        "v" => VIRTUAL_KEY(0x56),
        "w" => VIRTUAL_KEY(0x57),
        "x" => VIRTUAL_KEY(0x58),
        "y" => VIRTUAL_KEY(0x59),
        "z" => VIRTUAL_KEY(0x5A),

        // Numbers (0-9 are 0x30-0x39)
        "0" => VIRTUAL_KEY(0x30),
        "1" => VIRTUAL_KEY(0x31),
        "2" => VIRTUAL_KEY(0x32),
        "3" => VIRTUAL_KEY(0x33),
        "4" => VIRTUAL_KEY(0x34),
        "5" => VIRTUAL_KEY(0x35),
        "6" => VIRTUAL_KEY(0x36),
        "7" => VIRTUAL_KEY(0x37),
        "8" => VIRTUAL_KEY(0x38),
        "9" => VIRTUAL_KEY(0x39),

        // Special keys
        "space" => VK_SPACE,
        "return" | "enter" => VK_RETURN,
        "tab" => VK_TAB,
        "escape" | "esc" => VK_ESCAPE,
        "backspace" | "delete" => VK_BACK,
        "delete" => VK_DELETE,

        // Arrow keys
        "left" => VK_LEFT,
        "right" => VK_RIGHT,
        "up" => VK_UP,
        "down" => VK_DOWN,

        // Navigation keys
        "home" => VK_HOME,
        "end" => VK_END,
        "pageup" => VK_PRIOR,
        "pagedown" => VK_NEXT,

        // Modifier keys
        "shift" => VK_SHIFT,
        "control" | "ctrl" => VK_CONTROL,
        "alt" | "option" => VK_MENU,
        "command" | "windows" | "win" => VIRTUAL_KEY(0x5B),

        // Punctuation
        ";" => VIRTUAL_KEY(0xBA), // VK_OEM_1
        "=" => VIRTUAL_KEY(0xBB), // VK_OEM_PLUS
        "," => VIRTUAL_KEY(0xBC), // VK_OEM_COMMA
        "-" => VIRTUAL_KEY(0xBD), // VK_OEM_MINUS
        "." => VIRTUAL_KEY(0xBE), // VK_OEM_PERIOD
        "/" => VIRTUAL_KEY(0xBF), // VK_OEM_2
        "`" => VIRTUAL_KEY(0xC0), // VK_OEM_3
        "[" => VIRTUAL_KEY(0xDB), // VK_OEM_4
        "\\" => VIRTUAL_KEY(0xDC), // VK_OEM_5
        "]" => VIRTUAL_KEY(0xDD), // VK_OEM_6
        "'" => VIRTUAL_KEY(0xDE), // VK_OEM_7

        _ => return Err(anyhow::anyhow!("Unknown key: {}", key)),
    };

    Ok(vk)
}
