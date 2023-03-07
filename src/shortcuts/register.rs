//TODO : make linux compatible, add compile time optimisations
use windows::Win32::UI::Input::KeyboardAndMouse::{self, RegisterHotKey, HOT_KEY_MODIFIERS};

pub fn register_shortcut() {
    unsafe {
        RegisterHotKey(
            None,
            1,
            windows::Win32::UI::Input::KeyboardAndMouse::MOD_CONTROL,
            0x42,
        );
    }
    unsafe {
        RegisterHotKey(None, 1, KeyboardAndMouse::MOD_ALT, 0x43);
    }
}
