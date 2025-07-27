use std::{thread, time};
use win_hotkeys::{Hotkey, HotkeyManager, VKey};
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

fn main() {
    let hkm = HotkeyManager::current();

    hkm.register_hotkey(Hotkey::new(VKey::P, [VKey::Control], || {
        show_popup("Pomodoro Timer", "Pomodoro started! Focus for 25 minutes.");
        thread::spawn(|| {
            thread::sleep(time::Duration::from_secs(25 * 60));
            show_popup("Pomodoro Timer", "Time's up! Take a break.");
        });
    }))
    .unwrap();

    hkm.register_hotkey(Hotkey::new(VKey::S, [VKey::Control], || {
        show_popup("Pomodoro Timer", "Pomodoro stopped!");
    }))
    .unwrap();

    let event_loop_thread = HotkeyManager::start_keyboard_capturing().unwrap();
    event_loop_thread.join().unwrap(); // Block until the event loop thread exits
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
