use ime_core::KeyEvent;

#[cfg(windows)]
use windows::Win32::Foundation::WPARAM;

pub const VK_BACK: u32 = 0x08;
pub const VK_RETURN: u32 = 0x0D;
pub const VK_ESCAPE: u32 = 0x1B;
pub const VK_SPACE: u32 = 0x20;
pub const VK_0: u32 = 0x30;
pub const VK_9: u32 = 0x39;
pub const VK_A: u32 = 0x41;
pub const VK_Z: u32 = 0x5A;
pub const VK_F2: u32 = 0x71;

pub fn translate_virtual_key(virtual_key: u32) -> Option<KeyEvent> {
    match virtual_key {
        VK_BACK => Some(KeyEvent::Backspace),
        VK_RETURN => Some(KeyEvent::Enter),
        VK_ESCAPE => Some(KeyEvent::Escape),
        VK_SPACE => Some(KeyEvent::Space),
        VK_F2 => Some(KeyEvent::ToggleInputMode),
        VK_0..=VK_9 => Some(KeyEvent::Number((virtual_key - VK_0) as u8)),
        VK_A..=VK_Z => char::from_u32(virtual_key).map(KeyEvent::Char),
        _ => None,
    }
}

#[cfg(windows)]
pub fn translate_wparam(wparam: WPARAM) -> Option<KeyEvent> {
    translate_virtual_key(wparam.0 as u32)
}

#[cfg(test)]
mod tests {
    use super::{VK_A, VK_BACK, VK_ESCAPE, VK_F2, VK_RETURN, VK_SPACE, translate_virtual_key};
    use ime_core::KeyEvent;

    #[test]
    fn maps_alpha_virtual_keys_to_char_events() {
        assert_eq!(translate_virtual_key(VK_A), Some(KeyEvent::Char('A')));
    }

    #[test]
    fn maps_control_keys_to_engine_events() {
        assert_eq!(translate_virtual_key(VK_BACK), Some(KeyEvent::Backspace));
        assert_eq!(translate_virtual_key(VK_RETURN), Some(KeyEvent::Enter));
        assert_eq!(translate_virtual_key(VK_ESCAPE), Some(KeyEvent::Escape));
        assert_eq!(translate_virtual_key(VK_SPACE), Some(KeyEvent::Space));
        assert_eq!(translate_virtual_key(VK_F2), Some(KeyEvent::ToggleInputMode));
    }

    #[test]
    fn maps_number_keys_to_selection_events() {
        assert_eq!(translate_virtual_key(0x31), Some(KeyEvent::Number(1)));
        assert_eq!(translate_virtual_key(0x39), Some(KeyEvent::Number(9)));
    }

    #[test]
    fn ignores_unsupported_virtual_keys() {
        assert_eq!(translate_virtual_key(0x25), None);
    }
}
