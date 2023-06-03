use std::sync::mpsc::Receiver;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::RegisterHotKey;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY;

mod service_windows;

use crate::execute_precompiled;
use crate::logger::LoggerComponent;
use crate::runtime::keyboard::{
    convert_key_char_to_windows_virtual_key, convert_windows_virtual_key_code_to_key_char, KeyModes,
};
use crate::shortcuts_map::{read_shortcuts, ShortcutFile, ShortcutMap};

#[cfg(windows)]
use self::service_windows::init_service;

#[derive(Debug)]
struct Record {
    pub modes: Vec<KeyModes>,
    pub repeat: bool,
    pub key: char,
}

pub fn register_daemon() {
    //TODO run service based on platform

    if let Err(e) = init_service() {
        LoggerComponent::Daemon.log_error(e);
    }
}

pub fn run_daemon(shutdown_rx: Receiver<()>) {
    //TODO when does the daemon stop?
    LoggerComponent::Daemon.log(&format!("INIT DAEMON"), None);
    //test stop after 5 minutes

    let shortcuts = read_shortcuts();
    let mut records: Vec<Record> = Vec::new();
    let mut modes: Vec<KeyModes> = Vec::new();
    LoggerComponent::Daemon.log(&format!("Got SH"), None);
    let len = shortcuts.len();
    LoggerComponent::Daemon.log(&format!("SH len: {len}"), None);

    populate_records(&shortcuts, &mut records, &mut modes);
    register_records_with_windows(records);

    loop {
        //TODO uncomment

        // match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
        //     // Break the loop either upon stop or channel disconnect
        //     Ok(_) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,

        //     // Continue work if no events were received within the timeout
        //     Err(std::sync::mpsc::RecvTimeoutError::Timeout) => (),
        // };
        LoggerComponent::Daemon.log(&format!("linstening"), None);

        let mut lpmsg = MSG::default();
        unsafe { GetMessageW(&mut lpmsg, None, 0, 0) };
        LoggerComponent::Daemon.log(&format!("GOT MSG"), None);
        if lpmsg.message == WM_HOTKEY {
            //TODO : imporve , find a better way to do this
            let ho = (lpmsg.lParam.0 & 0xffff) as u32;
            let lo = (lpmsg.lParam.0 & 0xffff00) as u32 >> 16;
            //TODO : check for repeat

            let mut key_modes: Vec<KeyModes> = KeyModes::get_modes_from_u32(ho);

            key_modes.sort();
            let mut modes: Vec<&str> = key_modes.iter().map(|v| <&str>::from(v)).collect();

            let key_char = convert_windows_virtual_key_code_to_key_char(
                KeyboardAndMouse::VIRTUAL_KEY(lo as u16),
            );
            let key_char = key_char.to_string();
            modes.push(&key_char);

            let shortcut_file = shortcuts.get_file_from_key_iter(modes.into_iter().peekable());

            if shortcut_file.is_some() {
                let file_name = format!("{}.json", &shortcut_file.as_ref().unwrap().0);
                let res = execute_precompiled(&file_name);
                if res.is_err() {
                    LoggerComponent::Daemon.log(&format!("ERROR executing"), None);

                    eprintln!("ERROR executing: {}", res.unwrap_err())
                }
            } else {
                LoggerComponent::Daemon.log(
                    &format!("ERROR in daemon : no file with that shortcut found"),
                    None,
                );
                eprintln!("ERROR in daemon : no file with that shortcut found")
            }
        }
    }
}

fn populate_records(sm: &ShortcutMap, records: &mut Vec<Record>, modes: &mut Vec<KeyModes>) {
    for (key, value) in sm.iter() {
        if value.file.is_some() {
            let ShortcutFile(_, repeat) = value.file.as_ref().unwrap();
            records.push(Record {
                key: key.chars().next().unwrap(),
                modes: Vec::clone(modes),
                repeat: *repeat,
            })
        }
        if value.map.is_some() {
            modes.push(KeyModes::from(key));
            let map = &*value.map.as_ref().unwrap();
            populate_records(map, records, modes);
            modes.pop();
        }
    }
}

fn register_records_with_windows(records: Vec<Record>) {
    for rec in records {
        let mut mode = rec
            .modes
            .iter()
            .fold(KeyboardAndMouse::HOT_KEY_MODIFIERS(0), |mode, key| {
                mode | key.get_windows_key_mode()
            });
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
