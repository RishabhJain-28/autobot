use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::RegisterHotKey;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY;

use crate::execute_precompiled;
use crate::runtime::keyboard::{
    convert_key_char_to_windows_virtual_key, convert_windows_virtual_key_code_to_key_char, KeyModes,
};
use crate::shortcuts_map::{read_shortcuts, ShortcutFile, ShortcutMap};

#[derive(Debug)]
struct Record {
    pub mode: Vec<KeyModes>,
    pub repeat: bool,
    pub key: char,
}

pub fn run_daemon() {
    let shortcuts = read_shortcuts();
    let mut records: Vec<Record> = Vec::new();
    let mut modes: Vec<KeyModes> = Vec::new();

    iterate_shortcut_map(&shortcuts, &mut records, &mut modes);
    register_records_with_windows(records);
    loop {
        let mut lpmsg = MSG::default();
        unsafe { GetMessageW(&mut lpmsg, None, 0, 0) };
        if lpmsg.message == WM_HOTKEY {
            //TODO : imporve , find a better way to do this
            let ho = (lpmsg.lParam.0 & 0xffff) as u32;
            let lo = (lpmsg.lParam.0 & 0xffff00) as u32 >> 16;

            // //TODO:  make recursive, mode would be a vector
            //TODO : check for repeat
            let mode = KeyModes::from_windows_mode(KeyboardAndMouse::HOT_KEY_MODIFIERS(ho));
            let key_char = convert_windows_virtual_key_code_to_key_char(
                KeyboardAndMouse::VIRTUAL_KEY(lo as u16),
            );

            //TODO : FIX THIS CODE
            let smv = shortcuts.get(&String::from(&mode));

            if smv.is_some() {
                let smv = smv.unwrap();
                if smv.file.is_some() {
                    //TODO : RUN FILE
                }
                if smv.map.is_some() {
                    let file_smv = smv.map.as_ref().unwrap().get(&key_char.to_string());
                    if file_smv.is_some() {
                        let file_smv = file_smv.unwrap();
                        let file = &file_smv.file;
                        if file.is_some() {
                            let file_name = format!("{}.json", &file.as_ref().unwrap().0);
                            let res = execute_precompiled(&file_name);
                            if res.is_err() {
                                eprintln!("ERROR in '{} ' {}", file_name, res.unwrap_err())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn iterate_shortcut_map(sm: &ShortcutMap, records: &mut Vec<Record>, modes: &mut Vec<KeyModes>) {
    for (key, value) in sm.iter() {
        if value.file.is_some() {
            let ShortcutFile(_, repeat) = value.file.as_ref().unwrap();
            records.push(Record {
                key: key.chars().next().unwrap(),
                mode: Vec::clone(modes),
                repeat: *repeat,
            })
        }

        if value.map.is_some() {
            modes.push(KeyModes::from(key));
            let map = &*value.map.as_ref().unwrap();
            iterate_shortcut_map(map, records, modes);
            modes.pop();
        }
    }
}

//todo : fix codem add as a methid on reocrds struct
fn register_records_with_windows(records: Vec<Record>) {
    for rec in records {
        let mut mode = rec.mode.iter().next().unwrap().get_windows_key_mode();
        if !rec.repeat {
            mode = mode | KeyboardAndMouse::MOD_NOREPEAT
        }
        let key = rec.key;
        unsafe {
            RegisterHotKey(
                None,
                1,
                mode,
                convert_key_char_to_windows_virtual_key(key).0.into(),
            );
        }
    }
}
