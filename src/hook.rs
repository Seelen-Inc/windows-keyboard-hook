//! Provides a low-level implementation of a keyboard hook
//! using the Windows API. It captures keyboard events such as key presses
//! and releases, tracks the state of modifier keys, and communicates events
//! via channels to the rest of the application.

use crate::state::KeyboardState;
use crossbeam_channel::{unbounded, Receiver, RecvError, Sender};
use std::sync::{Mutex, OnceLock, RwLock};
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

/// Timeout for blocking key events, measured in milliseconds.
const TIMEOUT: Duration = Duration::from_millis(100);

/// Unassigned Virtual Key code used to suppress Windows Key events.
const SILENT_KEY: VIRTUAL_KEY = VIRTUAL_KEY(0xE8);

/// Channel sender used by hook proc to send keyboard events.
pub static HOOK_EVENT_TX: RwLock<Option<Sender<KeyboardEvent>>> = RwLock::new(None);

/// Channel receiver used to notify the hook on how to handle keyboard events.
static HOOK_RESPONSE_RX: RwLock<Option<Receiver<KeyAction>>> = RwLock::new(None);

/// Channel receiver used to notify hook proc to exit.
static HOOK_CONTROL_RX: RwLock<Option<Receiver<ControlFlow>>> = RwLock::new(None);

/// Bitmask object representing all pressed keys on keyboard.
static KEYBOARD_STATE: OnceLock<Mutex<KeyboardState>> = OnceLock::new();

/// Enum representing how to handle keypress.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyAction {
    Allow,
    Block,
    Replace,
}

/// Enum representing control flow signals for the hook thread.
#[derive(Debug, Copy, Clone, PartialEq)]
enum ControlFlow {
    Exit,
}

/// Enum representing keyboard events.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardEvent {
    KeyDown {
        /// The virtual key code of the key.
        vk_code: u16,
        /// The updated keyboard state due to this event.
        keyboard_state: KeyboardState,
    },
    KeyUp {
        /// The virtual key code of the key.
        key_code: u16,
        /// The updated keyboard state due to this event.
        keyboard_state: KeyboardState,
    },
}

/// Struct representing the keyboard hook interface
pub struct KeyboardHook {
    ke_rx: Receiver<KeyboardEvent>,
    action_tx: Sender<KeyAction>,
    cf_tx: Sender<ControlFlow>,
}

impl KeyboardHook {
    /// Receives a keyboard event from the hook.
    pub fn recv(&self) -> Result<KeyboardEvent, RecvError> {
        self.ke_rx.recv()
    }

    /// Blocks or unblocks the propagation of the key event.
    pub fn key_action(&self, value: KeyAction) {
        self.action_tx.send(value).unwrap();
    }

    /// Signals the hook thread to exit.
    pub fn exit(&self) {
        self.cf_tx.send(ControlFlow::Exit).unwrap();
    }
}

/// Starts the keyboard hook in a separate thread.
///
/// # Returns
/// A `KeyboardHook` instance to interact with the hook (e.g., receiving events, blocking keys).
pub fn start() -> KeyboardHook {
    // Create channels
    let (ke_tx, ke_rx) = unbounded();
    let (action_tx, action_rx) = unbounded();
    let (cf_tx, cf_rx) = unbounded();

    // Set static channel variables
    let mut hook_event_tx = HOOK_EVENT_TX.write().unwrap();
    *hook_event_tx = Some(ke_tx);
    let mut hook_response_rx = HOOK_RESPONSE_RX.write().unwrap();
    *hook_response_rx = Some(action_rx);
    let mut hook_control_tx = HOOK_CONTROL_RX.write().unwrap();
    *hook_control_tx = Some(cf_rx);

    // Create/clear keyboard state
    let mutex = KEYBOARD_STATE.get_or_init(|| Mutex::new(KeyboardState::new()));
    let mut state = mutex.lock().unwrap();
    state.clear();

    unsafe {
        thread::spawn(|| {
            let hhook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), None, 0).unwrap();
            if let Some(cf_rx) = &*HOOK_CONTROL_RX.read().unwrap() {
                loop {
                    let mut msg = MSG::default();
                    if GetMessageW(&mut msg, None, 0, 0).into() {
                        let _ = TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }

                    if let Ok(cf) = cf_rx.try_recv() {
                        match cf {
                            ControlFlow::Exit => {
                                let mut hook_event_tx = HOOK_EVENT_TX.write().unwrap();
                                *hook_event_tx = None;
                                let mut hook_response_rx = HOOK_RESPONSE_RX.write().unwrap();
                                *hook_response_rx = None;
                                let mut hook_control_tx = HOOK_CONTROL_RX.write().unwrap();
                                *hook_control_tx = None;
                                UnhookWindowsHookEx(hhook).unwrap();
                                break;
                            }
                        }
                    }
                }
            }
        });
    }

    KeyboardHook {
        ke_rx,
        action_tx,
        cf_tx,
    }
}

/// Updates global keyboard state for given virtual key code.
fn update_keyboard_state(vk_code: u16) {
    let mutex = KEYBOARD_STATE.get();
    let mut keyboard = mutex.unwrap().lock().unwrap();
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

/// Hook procedure for handling keyboard events.
unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let event_guard = HOOK_EVENT_TX.read().unwrap();
        let event_tx = event_guard.as_ref().unwrap();
        let response_guard = HOOK_RESPONSE_RX.read().unwrap();
        let response_rx = response_guard.as_ref().unwrap();

        let event_type = wparam.0 as u32;
        let vk_code = (*(lparam.0 as *const KBDLLHOOKSTRUCT)).vkCode as u16;
        if vk_code == SILENT_KEY.0 {
            return CallNextHookEx(None, code, wparam, lparam);
        }

        match event_type {
            // We only care about key down events
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                update_keyboard_state(vk_code);
                event_tx
                    .send(KeyboardEvent::KeyDown {
                        vk_code,
                        keyboard_state: *KEYBOARD_STATE.get().unwrap().lock().unwrap(),
                    })
                    .unwrap();

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
