use serde::{Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse;
// {
//     HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT,

// };

//TODO : remove partial eq
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]

pub enum KeyModes {
    ALT,
    CTRL,
    SHIFT,
}

impl From<&KeyModes> for &str {
    fn from(value: &KeyModes) -> Self {
        match value {
            KeyModes::ALT => &"alt",
            KeyModes::CTRL => &"ctrl",
            KeyModes::SHIFT => &"shift",
        }
    }
}
impl From<&KeyModes> for String {
    fn from(value: &KeyModes) -> Self {
        match value {
            KeyModes::ALT => "alt".to_string(),
            KeyModes::CTRL => "ctrl".to_string(),
            KeyModes::SHIFT => "shift".to_string(),
        }
    }
}

impl From<&String> for KeyModes {
    //TODO : change key literals to consts
    fn from(value: &String) -> Self {
        match value.as_str() {
            "alt" => Self::ALT,
            "ctrl" => Self::CTRL,
            "shift" => Self::SHIFT,
            _ => {
                panic!("Invalid key mode")
            }
        }
    }
}

impl KeyModes {
    pub fn get_windows_key_mode(&self) -> KeyboardAndMouse::HOT_KEY_MODIFIERS {
        match self {
            Self::ALT => KeyboardAndMouse::MOD_ALT,
            Self::CTRL => KeyboardAndMouse::MOD_CONTROL,
            Self::SHIFT => KeyboardAndMouse::MOD_SHIFT,
        }
    }
    pub fn from_windows_mode(mode: KeyboardAndMouse::HOT_KEY_MODIFIERS) -> Self {
        match mode {
            KeyboardAndMouse::MOD_ALT => KeyModes::ALT,
            KeyboardAndMouse::MOD_CONTROL => KeyModes::CTRL,
            KeyboardAndMouse::MOD_SHIFT => KeyModes::SHIFT,
            _ => {
                panic!("invalid mode value")
            }
        }
    }
}

pub fn convert_key_char_to_windows_virtual_key(key: char) -> KeyboardAndMouse::VIRTUAL_KEY {
    match key {
        'a' => KeyboardAndMouse::VK_A,
        'b' => KeyboardAndMouse::VK_B,
        'c' => KeyboardAndMouse::VK_C,
        'd' => KeyboardAndMouse::VK_D,
        'e' => KeyboardAndMouse::VK_E,
        'f' => KeyboardAndMouse::VK_F,
        'g' => KeyboardAndMouse::VK_G,
        'h' => KeyboardAndMouse::VK_H,
        'i' => KeyboardAndMouse::VK_I,
        'j' => KeyboardAndMouse::VK_J,
        'k' => KeyboardAndMouse::VK_K,
        'l' => KeyboardAndMouse::VK_L,
        'm' => KeyboardAndMouse::VK_M,
        'n' => KeyboardAndMouse::VK_N,
        'o' => KeyboardAndMouse::VK_O,
        'p' => KeyboardAndMouse::VK_P,
        'q' => KeyboardAndMouse::VK_Q,
        'r' => KeyboardAndMouse::VK_R,
        's' => KeyboardAndMouse::VK_S,
        't' => KeyboardAndMouse::VK_T,
        'u' => KeyboardAndMouse::VK_U,
        'v' => KeyboardAndMouse::VK_V,
        'w' => KeyboardAndMouse::VK_W,
        'x' => KeyboardAndMouse::VK_X,
        'z' => KeyboardAndMouse::VK_Z,
        '0' => KeyboardAndMouse::VK_0,
        '1' => KeyboardAndMouse::VK_1,
        '2' => KeyboardAndMouse::VK_2,
        '3' => KeyboardAndMouse::VK_3,
        '4' => KeyboardAndMouse::VK_4,
        '5' => KeyboardAndMouse::VK_5,
        '6' => KeyboardAndMouse::VK_6,
        '7' => KeyboardAndMouse::VK_7,
        '8' => KeyboardAndMouse::VK_8,
        '9' => KeyboardAndMouse::VK_9,

        _ => {
            panic!("Invalid key ")
        }
    }
}

pub fn convert_windows_virtual_key_code_to_key_char(vkey: KeyboardAndMouse::VIRTUAL_KEY) -> char {
    match vkey {
        KeyboardAndMouse::VK_A => 'a',
        KeyboardAndMouse::VK_B => 'b',
        KeyboardAndMouse::VK_C => 'c',
        KeyboardAndMouse::VK_D => 'd',
        KeyboardAndMouse::VK_E => 'e',
        KeyboardAndMouse::VK_F => 'f',
        KeyboardAndMouse::VK_G => 'g',
        KeyboardAndMouse::VK_H => 'h',
        KeyboardAndMouse::VK_I => 'i',
        KeyboardAndMouse::VK_J => 'j',
        KeyboardAndMouse::VK_K => 'k',
        KeyboardAndMouse::VK_L => 'l',
        KeyboardAndMouse::VK_M => 'm',
        KeyboardAndMouse::VK_N => 'n',
        KeyboardAndMouse::VK_O => 'o',
        KeyboardAndMouse::VK_P => 'p',
        KeyboardAndMouse::VK_Q => 'q',
        KeyboardAndMouse::VK_R => 'r',
        KeyboardAndMouse::VK_S => 's',
        KeyboardAndMouse::VK_T => 't',
        KeyboardAndMouse::VK_U => 'u',
        KeyboardAndMouse::VK_V => 'v',
        KeyboardAndMouse::VK_W => 'w',
        KeyboardAndMouse::VK_X => 'x',
        KeyboardAndMouse::VK_Z => 'z',
        KeyboardAndMouse::VK_0 => '0',
        KeyboardAndMouse::VK_1 => '1',
        KeyboardAndMouse::VK_2 => '2',
        KeyboardAndMouse::VK_3 => '3',
        KeyboardAndMouse::VK_4 => '4',
        KeyboardAndMouse::VK_5 => '5',
        KeyboardAndMouse::VK_6 => '6',
        KeyboardAndMouse::VK_7 => '7',
        KeyboardAndMouse::VK_8 => '8',
        KeyboardAndMouse::VK_9 => '9',

        _ => {
            panic!("Invalid key ")
        }
    }
}
