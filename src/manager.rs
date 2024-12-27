//! Defines the `HotkeyManager`, which manages the registration,
//! unregistration, and execution of hotkeys. It also handles the main event
//! loop that listens for keyboard events and invokes associated callbacks.

use crate::error::WHKError;
use crate::error::WHKError::RegistrationFailed;
use crate::hook;
use crate::hook::KeyboardEvent;
use crate::hotkey::Hotkey;
use crate::keys::{ModKey, VKey};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Manages the lifecycle of hotkeys, including their registration, unregistration, and execution.
///
/// The `HotkeyManager` listens for keyboard events and triggers the corresponding
/// hotkey callbacks when events match registered hotkeys.
pub struct HotkeyManager<T> {
    hotkeys: HashMap<u16, Vec<Hotkey<T>>>,
    interrupt: Arc<AtomicBool>,
    callback_results_channel: Option<Sender<T>>,
}

impl<T> Default for HotkeyManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HotkeyManager<T> {
    pub fn new() -> HotkeyManager<T> {
        Self {
            hotkeys: HashMap::new(),
            interrupt: Arc::new(AtomicBool::new(false)),
            callback_results_channel: None,
        }
    }

    /// Registers a new hotkey.
    pub fn register_hotkey(
        &mut self,
        trigger_key: VKey,
        mod_keys: &[ModKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<i32, WHKError> {
        let hotkey = Hotkey::new(&trigger_key, mod_keys, callback);
        let id = hotkey.id;
        if self
            .hotkeys
            .values()
            .any(|vec| vec.iter().any(|hotkey| hotkey.id == id))
        {
            return Err(RegistrationFailed);
        }
        let entry = self.hotkeys.entry(trigger_key.to_vk_code()).or_default();
        entry.push(hotkey);
        Ok(id)
    }

    /// Unregisters a hotkey by its unique id
    pub fn unregister_hotkey(&mut self, hotkey_id: i32) {
        for vec in self.hotkeys.values_mut() {
            vec.retain(|hotkey| hotkey.id != hotkey_id);
        }
    }

    /// Unregisters all hotkeys
    pub fn unregister_all(&mut self) {
        self.hotkeys.clear();
    }

    /// Registers a channel for callback results to be sent into
    pub fn register_channel(&mut self, channel: Sender<T>) {
        self.callback_results_channel = Some(channel);
    }

    /// Runs the main event loop to listen for keyboard events.
    ///
    /// This method blocks and processes keyboard events until interrupted.
    /// It matches events against registered hotkeys and executes the corresponding callbacks.
    pub fn event_loop(&mut self) {
        let hook = hook::start();
        while !self.interrupt.load(Ordering::Relaxed) {
            if let Ok(event) = hook.recv() {
                let (key_code, ctrl, shift, alt, win) = match event {
                    KeyboardEvent::KeyDown {
                        key_code,
                        ctrl,
                        shift,
                        alt,
                        win,
                    } => (key_code, ctrl, shift, alt, win),
                    _ => continue,
                };

                let mut modifiers = ModKey::mod_mask_from_bool(ctrl, shift, alt, win);
                if let Ok(mod_key) = ModKey::from_vk_code(key_code) {
                    modifiers ^= mod_key.to_mod_bit();
                }
                let event_id = Hotkey::<T>::generate_hotkey_id(key_code, modifiers);

                let mut found = false;
                if let Some(hotkeys) = self.hotkeys.get_mut(&key_code) {
                    for hotkey in hotkeys {
                        if hotkey.id == event_id {
                            hook.block(true);
                            let result = (hotkey.callback)();
                            if let Some(callback_result_channel) = &self.callback_results_channel {
                                callback_result_channel.send(result).unwrap();
                            }
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    hook.block(false);
                }
            }
        }
        hook.exit();
    }

    /// Signals the `HotkeyManager` to interrupt its event loop.
    pub fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle {
            interrupt_handle: Arc::clone(&self.interrupt),
        }
    }
}

/// A handle for signaling the `HotkeyManager` to stop its event loop.
///
/// The `InterruptHandle` is used to gracefully interrupt the event loop by sending
/// a control signal. This allows the `HotkeyManager` to clean up resources and stop
/// processing keyboard events.
pub struct InterruptHandle {
    interrupt_handle: Arc<AtomicBool>,
}

unsafe impl Sync for InterruptHandle {}

unsafe impl Send for InterruptHandle {}

impl InterruptHandle {
    pub fn interrupt(&self) {
        use crate::hook::{KeyboardEvent, HOOK_CHANNELS};
        let dummy_event = KeyboardEvent::KeyDown {
            key_code: 0,
            ctrl: false,
            shift: false,
            alt: false,
            win: false,
        };
        self.interrupt_handle.store(true, Ordering::Relaxed);
        let hook_channels = HOOK_CHANNELS.read().unwrap();
        if let Some((ke_tx, _, _)) = &*hook_channels {
            ke_tx.send(dummy_event).unwrap();
        }
    }
}
