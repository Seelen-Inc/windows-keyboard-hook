//! Provides a low-level implementation of a keyboard hook
//! using the Windows API. It captures keyboard events such as key presses
//! and releases, tracks the state of modifier keys, and communicates events
//! via channels to the rest of the application.

use crate::error::{Result, WHKError};
use crate::events::{KeyAction, KeyboardInputEvent};
use crate::state::KeyboardState;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, PostThreadMessageW, SetWindowsHookExW,
    TranslateMessage, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_QUIT, WM_SYSKEYDOWN,
};

/// Timeout for blocking key events, measured in milliseconds.
const TIMEOUT: Duration = Duration::from_millis(250);

/// Unassigned Virtual Key code used to suppress Windows Key events.
const SILENT_KEY: VIRTUAL_KEY = VIRTUAL_KEY(0xE8);

/// Bitmask object representing all pressed keys on keyboard.
static KEYBOARD_STATE: LazyLock<Arc<Mutex<KeyboardState>>> = LazyLock::new(|| {
    let mutex = Mutex::new(KeyboardState::new());
    Arc::new(mutex)
});

static STARTED: AtomicBool = AtomicBool::new(false);
static HOOK_THREAD_ID: AtomicU32 = AtomicU32::new(0);

/// Starts the keyboard hook thread.
pub fn start() -> Result<()> {
    if STARTED.load(Ordering::Relaxed) {
        return Ok(());
    }

    // Create/clear keyboard state
    KEYBOARD_STATE.lock().unwrap().clear();

    let (tx, rx) = crossbeam_channel::unbounded::<bool>();
    thread::spawn(move || unsafe {
        let Ok(_hhook) = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), None, 0) else {
            tx.send(false).unwrap();
            return;
        };

        tx.send(true).unwrap();
        HOOK_THREAD_ID.store(GetCurrentThreadId(), Ordering::Relaxed);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    });

    if rx.recv()? {
        STARTED.store(true, Ordering::Relaxed);
        Ok(())
    } else {
        Err(WHKError::StartupFailed)
    }
}

pub fn stop() {
    let thread_id = HOOK_THREAD_ID.load(Ordering::Relaxed);
    if !STARTED.load(Ordering::Relaxed) || thread_id == 0 {
        return;
    }
    unsafe {
        let _ = PostThreadMessageW(thread_id, WM_QUIT, WPARAM::default(), LPARAM::default());
    }
}

/// Hook procedure for handling keyboard events.
unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let event_type = wparam.0 as u32;
        let vk_code = (*(lparam.0 as *const KBDLLHOOKSTRUCT)).vkCode as u16;
        if vk_code == SILENT_KEY.0 {
            return CallNextHookEx(None, code, wparam, lparam);
        }

        match event_type {
            // We only care about key down events
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                let response_rx = KeyAction::reciever();
                // Clear the actions channel of any previous action
                while response_rx.try_recv().is_ok() {}

                update_keyboard_state(vk_code);
                KeyboardInputEvent::send(KeyboardInputEvent::KeyDown {
                    vk_code,
                    keyboard_state: *KEYBOARD_STATE.lock().unwrap(),
                });

                // Wait for response on how to handle event
                if let Ok(action) = response_rx.recv_timeout(TIMEOUT) {
                    match action {
                        KeyAction::Block => {
                            return LRESULT(1);
                        }
                        KeyAction::Replace => {
                            send_silent_key();
                            return LRESULT(1);
                        }
                        KeyAction::Allow => {}
                    }
                }
            }
            _ => {}
        };
    }
    CallNextHookEx(None, code, wparam, lparam)
}

/// Updates global keyboard state for given virtual key code.
fn update_keyboard_state(vk_code: u16) {
    let mut keyboard = KEYBOARD_STATE.lock().unwrap();
    keyboard.sync();
    keyboard.keydown(vk_code);
}

/// Sends a keydown and keyup event for Unassigned Virtual Key 0xE8.
unsafe fn send_silent_key() {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: SILENT_KEY,
                    wScan: 0,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: SILENT_KEY,
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    SendInput(&inputs, size_of::<INPUT>() as i32);
}
