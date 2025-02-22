use std::{thread, time};
use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register_hotkey(VKey::P, &[VKey::Control], || {
        show_popup("Pomodoro Timer", "Pomodoro started! Focus for 25 minutes.");
        thread::spawn(|| {
            thread::sleep(time::Duration::from_secs(25 * 60));
            show_popup("Pomodoro Timer", "Time's up! Take a break.");
        });
    })
    .unwrap();

    hkm.register_hotkey(VKey::S, &[VKey::Control], || {
        show_popup("Pomodoro Timer", "Pomodoro stopped!");
    })
    .unwrap();

    hkm.event_loop();
}

fn show_popup(title: &str, message: &str) {
    unsafe {
        MessageBoxW(
            Some(HWND(std::ptr::null_mut())),
            PCWSTR(to_wide_string(message).as_ptr()),
            PCWSTR(to_wide_string(title).as_ptr()),
            MB_OK,
        );
    }
}

fn to_wide_string(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
