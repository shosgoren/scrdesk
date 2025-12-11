use super::InputSimulator;
use crate::protocol::{KeyModifiers, MouseButton};
use anyhow::Result;
use core_graphics::event::{CGEvent, CGEventTapLocation, CGEventType, CGKeyCode, CGMouseButton, EventField, CGEventField};
use core_graphics::geometry::CGPoint;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

pub struct MacOSSimulator {
    // CGEventSource is not Send, so we create it on demand
}

impl MacOSSimulator {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    fn create_event_source(&self) -> Result<CGEventSource> {
        CGEventSource::new(CGEventSourceStateID::CombinedSessionState)
            .map_err(|_| anyhow::anyhow!("Failed to create CGEventSource"))
    }

    fn map_mouse_button(&self, button: MouseButton) -> CGMouseButton {
        match button {
            MouseButton::Left => CGMouseButton::Left,
            MouseButton::Right => CGMouseButton::Right,
            MouseButton::Middle => CGMouseButton::Center,
            _ => CGMouseButton::Left,
        }
    }
}

impl InputSimulator for MacOSSimulator {
    fn simulate_mouse_move(&self, x: i32, y: i32) -> Result<()> {
        let source = self.create_event_source()?;
        let event = CGEvent::new_mouse_event(
            source,
            CGEventType::MouseMoved,
            CGPoint::new(x as f64, y as f64),
            CGMouseButton::Left,
        ).map_err(|_| anyhow::anyhow!("Failed to create mouse move event"))?;

        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn simulate_mouse_button(&self, button: MouseButton, pressed: bool) -> Result<()> {
        let source = self.create_event_source()?;
        let cg_button = self.map_mouse_button(button);
        let event_type = if pressed {
            match button {
                MouseButton::Left => CGEventType::LeftMouseDown,
                MouseButton::Right => CGEventType::RightMouseDown,
                _ => CGEventType::OtherMouseDown,
            }
        } else {
            match button {
                MouseButton::Left => CGEventType::LeftMouseUp,
                MouseButton::Right => CGEventType::RightMouseUp,
                _ => CGEventType::OtherMouseUp,
            }
        };

        // Get current mouse position
        let location = CGEvent::new(source.clone())
            .map_err(|_| anyhow::anyhow!("Failed to get mouse location"))?
            .location();

        let event = CGEvent::new_mouse_event(
            source,
            event_type,
            location,
            cg_button,
        ).map_err(|_| anyhow::anyhow!("Failed to create mouse button event"))?;

        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn simulate_mouse_scroll(&self, _delta_x: i32, delta_y: i32) -> Result<()> {
        // core-graphics 0.23 doesn't have scroll event API
        // Use mouse wheel event instead
        let source = self.create_event_source()?;
        let event = CGEvent::new_mouse_event(
            source,
            CGEventType::ScrollWheel,
            CGPoint::new(0.0, 0.0),
            CGMouseButton::Left,
        ).map_err(|_| anyhow::anyhow!("Failed to create scroll event"))?;

        // Set scroll delta (field 11 is scroll wheel delta axis 1)
        event.set_integer_value_field(EventField::SCROLL_WHEEL_EVENT_DELTA_AXIS_1, delta_y as i64);

        event.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn simulate_key(&self, key: &str, pressed: bool, modifiers: KeyModifiers) -> Result<()> {
        // Map key string to CGKeyCode
        let keycode = map_key_to_keycode(key)?;

        let source = self.create_event_source()?;
        let event = CGEvent::new_keyboard_event(
            source,
            keycode,
            pressed,
        ).map_err(|_| anyhow::anyhow!("Failed to create keyboard event"))?;

        // Set modifiers
        let mut flags: u64 = 0;
        if modifiers.shift {
            flags |= 0x20000; // kCGEventFlagMaskShift
        }
        if modifiers.ctrl {
            flags |= 0x40000; // kCGEventFlagMaskControl
        }
        if modifiers.alt {
            flags |= 0x80000; // kCGEventFlagMaskAlternate
        }
        if modifiers.meta {
            flags |= 0x100000; // kCGEventFlagMaskCommand
        }

        event.set_flags(core_graphics::event::CGEventFlags::from_bits_truncate(flags));
        event.post(CGEventTapLocation::HID);
        Ok(())
    }
}

fn map_key_to_keycode(key: &str) -> Result<CGKeyCode> {
    // Basic key mapping
    let keycode = match key.to_lowercase().as_str() {
        "a" => 0x00,
        "s" => 0x01,
        "d" => 0x02,
        "f" => 0x03,
        "h" => 0x04,
        "g" => 0x05,
        "z" => 0x06,
        "x" => 0x07,
        "c" => 0x08,
        "v" => 0x09,
        "b" => 0x0B,
        "q" => 0x0C,
        "w" => 0x0D,
        "e" => 0x0E,
        "r" => 0x0F,
        "y" => 0x10,
        "t" => 0x11,
        "1" => 0x12,
        "2" => 0x13,
        "3" => 0x14,
        "4" => 0x15,
        "6" => 0x16,
        "5" => 0x17,
        "=" => 0x18,
        "9" => 0x19,
        "7" => 0x1A,
        "-" => 0x1B,
        "8" => 0x1C,
        "0" => 0x1D,
        "]" => 0x1E,
        "o" => 0x1F,
        "u" => 0x20,
        "[" => 0x21,
        "i" => 0x22,
        "p" => 0x23,
        "return" | "enter" => 0x24,
        "l" => 0x25,
        "j" => 0x26,
        "'" => 0x27,
        "k" => 0x28,
        ";" => 0x29,
        "\\" => 0x2A,
        "," => 0x2B,
        "/" => 0x2C,
        "n" => 0x2D,
        "m" => 0x2E,
        "." => 0x2F,
        "tab" => 0x30,
        "space" => 0x31,
        "`" => 0x32,
        "delete" | "backspace" => 0x33,
        "escape" | "esc" => 0x35,
        "command" => 0x37,
        "shift" => 0x38,
        "capslock" => 0x39,
        "option" | "alt" => 0x3A,
        "control" | "ctrl" => 0x3B,
        "left" => 0x7B,
        "right" => 0x7C,
        "down" => 0x7D,
        "up" => 0x7E,
        _ => return Err(anyhow::anyhow!("Unknown key: {}", key)),
    };

    Ok(keycode)
}
