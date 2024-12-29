//! Provides a low-level implementation of a keyboard hook
//! using the Windows API. It captures keyboard events such as key presses
//! and releases, tracks the state of modifier keys, and communicates events
//! via channels to the rest of the application.
//!
//! ## Features
//! - Tracks modifier keys (`Ctrl`, `Shift`, `Alt`, `Win`) using atomic flags.
//! - Sends `KeyboardEvent` objects for key down and key up events.
//! - Supports blocking or propagating key events based on user-defined logic.

use crossbeam_channel::{unbounded, Receiver, RecvError, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    VIRTUAL_KEY, VK_CONTROL, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_MENU, VK_RCONTROL,
    VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
    UnhookWindowsHookEx, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
    WM_SYSKEYUP,
};

/// Timeout for blocking key events, measured in milliseconds.
const TIMEOUT: Duration = Duration::from_millis(100);

/// Shared state for hook channels.
pub static HOOK_CHANNELS: RwLock<
    Option<(
        Sender<KeyboardEvent>,
        Receiver<KeyAction>,
        Receiver<ControlFlow>,
    )>,
> = RwLock::new(None);

/// Unassigned Virtual Key code used to suppress Windows Key events
const SILENT_KEY: VIRTUAL_KEY = VIRTUAL_KEY(0xE8);

/// Atomic flags for tracking modifier key states.
static CTRL: AtomicBool = AtomicBool::new(false);
static SHIFT: AtomicBool = AtomicBool::new(false);
static ALT: AtomicBool = AtomicBool::new(false);
static WIN: AtomicBool = AtomicBool::new(false);

/// Enum representing how to handle keypress
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyAction {
    Allow,
    Block,
    Replace,
}

/// Enum representing control flow signals for the hook thread.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ControlFlow {
    Exit,
}

/// Enum representing keyboard events.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardEvent {
    KeyDown {
        /// The virtual key code of the key.
        key_code: u16,
        /// Whether the Ctrl modifier was active.
        ctrl: bool,
        /// Whether the Shift modifier was active.
        shift: bool,
        /// Whether the Alt modifier was active.
        alt: bool,
        /// Whether the Win modifier was active.
        win: bool,
    },
    KeyUp {
        /// The virtual key code of the key
        key_code: u16,
        ctrl: bool,
        shift: bool,
        alt: bool,
        win: bool,
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

    /// Blocks or unblocks the propagation of the next key event.
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
    let (ke_tx, ke_rx) = unbounded();
    let (action_tx, action_rx) = unbounded();
    let (cf_tx, cf_rx) = unbounded();

    let mut hook_channels = HOOK_CHANNELS.write().unwrap();
    *hook_channels = Some((ke_tx, action_rx, cf_rx));

    unsafe {
        thread::spawn(|| {
            let hhook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), None, 0).unwrap();
            let hook_channels = HOOK_CHANNELS.read().unwrap();
            if let Some((_, _, cf_rx)) = &*hook_channels {
                loop {
                    let mut msg = MSG::default();
                    if GetMessageW(&mut msg, None, 0, 0).into() {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }

                    if let Ok(cf) = cf_rx.try_recv() {
                        match cf {
                            ControlFlow::Exit => {
                                let mut hook_channels = HOOK_CHANNELS.write().unwrap();
                                *hook_channels = None;
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

/// Updates the modifier state for the given virtual key code.
fn update_modifier_state(key: u16, state: bool) {
    match key {
        k if k == VK_CONTROL.0 || k == VK_LCONTROL.0 || k == VK_RCONTROL.0 => {
            CTRL.store(state, Ordering::Relaxed)
        }
        k if k == VK_SHIFT.0 || k == VK_LSHIFT.0 || k == VK_RSHIFT.0 => {
            SHIFT.store(state, Ordering::Relaxed)
        }
        k if k == VK_MENU.0 || k == VK_LMENU.0 || k == VK_RMENU.0 => {
            ALT.store(state, Ordering::Relaxed)
        }
        k if k == VK_LWIN.0 || k == VK_RWIN.0 => WIN.store(state, Ordering::Relaxed),
        _ => {}
    }
}

/// Hook procedure for handling keyboard events.
unsafe extern "system" fn hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        let hook_channels = HOOK_CHANNELS.read().unwrap();
        if let Some((ke_tx, action_rx, _)) = &*hook_channels {
            let kbd_hook = lparam.0 as *const KBDLLHOOKSTRUCT;
            let key = (*kbd_hook).vkCode as u16;
            let event = wparam.0 as u32;
            let mut check_block = false;

            if key == SILENT_KEY.0 {
                return CallNextHookEx(None, code, wparam, lparam);
            }

            match event {
                WM_KEYDOWN | WM_SYSKEYDOWN => {
                    update_modifier_state(key, true);
                    check_block = true;
                    ke_tx
                        .send(KeyboardEvent::KeyDown {
                            key_code: key,
                            ctrl: CTRL.load(Ordering::SeqCst),
                            shift: SHIFT.load(Ordering::SeqCst),
                            alt: ALT.load(Ordering::SeqCst),
                            win: WIN.load(Ordering::SeqCst),
                        })
                        .unwrap();
                }
                WM_KEYUP | WM_SYSKEYUP => {
                    update_modifier_state(key, false);
                }
                _ => {}
            };
            if check_block {
                if let Ok(action) = action_rx.recv_timeout(TIMEOUT) {
                    match action {
                        KeyAction::Block => {
                            return LRESULT(1);
                        }
                        KeyAction::Replace => {
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
                            return LRESULT(1);
                        }
                        KeyAction::Allow => {}
                    }
                }
            }
        }
    }
    CallNextHookEx(None, code, wparam, lparam)
}
