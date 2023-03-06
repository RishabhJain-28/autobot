use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_C, VK_MENU};
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::MSG;
use windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY;
use windows::{
    h,
    Win32::UI::Input::{
        GetCurrentInputMessageSource,
        KeyboardAndMouse::{RegisterHotKey, HOT_KEY_MODIFIERS},
    },
};
pub fn run_daemon() {
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
    loop {
        let mut lpmsg = MSG::default();
        unsafe { GetMessageW(&mut lpmsg, None, 0, 0) };
        if lpmsg.message == WM_HOTKEY {
            println!("{:?}", lpmsg.lParam);
            //TODO : imporve , find a better way to do this
            let ho = (lpmsg.lParam.0 & 0xffff) as u32;
            let lo = (lpmsg.lParam.0 & 0xffff00) as u32 >> 16;

            if lo as u16 == VK_C.0 && ho == KeyboardAndMouse::MOD_ALT.0 {
                println!("b + alt")
            }
        }
    }
}
