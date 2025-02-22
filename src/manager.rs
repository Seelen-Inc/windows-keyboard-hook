//! Defines the `HotkeyManager`, which manages the registration,
//! unregistration, and execution of hotkeys. It also handles the main event
//! loop that listens for keyboard events and invokes associated callbacks.

use crate::error::WHKError;
use crate::error::WHKError::RegistrationFailed;
use crate::hook;
use crate::hook::{KeyAction, KeyboardEvent};
use crate::hotkey::Hotkey;
use crate::state::KeyboardState;
use crate::VKey;
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Manages the lifecycle of hotkeys, including their registration, unregistration, and execution.
///
/// The `HotkeyManager` listens for keyboard events and triggers the corresponding
/// hotkey callbacks when events match registered
/// hotkeys.
/// # Type Parameters
/// - `T`: The return type of the hotkey callbacks.
pub struct HotkeyManager<T> {
    hotkeys: HashMap<u16, Vec<Hotkey<T>>>,
    paused_ids: Vec<i32>,
    paused: Arc<AtomicBool>,
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
            paused_ids: Vec::new(),
            paused: Arc::new(AtomicBool::new(false)),
            interrupt: Arc::new(AtomicBool::new(false)),
            callback_results_channel: None,
        }
    }

    /// Registers a new hotkey.
    pub fn register_hotkey(
        &mut self,
        trigger_key: VKey,
        mod_keys: &[VKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<i32, WHKError> {
        let hotkey = Hotkey::new(trigger_key, mod_keys, callback);
        let id = hotkey.generate_id();

        // Check if already exists
        if self
            .hotkeys
            .values()
            .any(|vec| vec.iter().any(|hotkey| hotkey.generate_id() == id))
        {
            return Err(RegistrationFailed);
        }

        // Add hotkey and return id
        self.hotkeys
            .entry(trigger_key.to_vk_code())
            .or_default()
            .push(hotkey);
        Ok(id)
    }

    /// Unregisters a hotkey by its unique id.
    pub fn unregister_hotkey(&mut self, hotkey_id: i32) {
        for vec in self.hotkeys.values_mut() {
            vec.retain(|hotkey| hotkey.generate_id() != hotkey_id);
        }
        self.paused_ids.retain(|id| *id != hotkey_id);
    }

    /// Unregisters all hotkeys.
    pub fn unregister_all(&mut self) {
        self.hotkeys.clear();
        self.paused_ids.clear();
    }

    /// Registers a hotkey that will toggle the paused state of the
    /// `HotkeyManager`. When paused, only registered pause hotkeys
    /// will be allowed to trigger.
    pub fn register_pause_hotkey(
        &mut self,
        trigger_key: VKey,
        mod_keys: &[VKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<i32, WHKError> {
        let paused = Arc::clone(&self.paused);
        let wrapped_callback = move || {
            let was_paused = paused.load(Ordering::Relaxed);
            paused.store(!was_paused, Ordering::Relaxed);
            callback()
        };
        let id = self.register_hotkey(trigger_key, mod_keys, wrapped_callback)?;
        self.paused_ids.push(id);
        Ok(id)
    }

    /// Registers a channel for callback results to be sent into.
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
                let (key_code, state) = match event {
                    KeyboardEvent::KeyDown {
                        vk_code: key_code,
                        keyboard_state: state,
                    } => (key_code, state),
                    _ => continue,
                };

                let mut found = false;
                if let Some(hotkeys) = self.hotkeys.get_mut(&key_code) {
                    for hotkey in hotkeys {
                        if self.paused.load(Ordering::Relaxed)
                            && !self.paused_ids.contains(&hotkey.generate_id())
                        {
                            continue;
                        }
                        if hotkey.is_trigger_state(state) {
                            if state.is_down(VKey::LWin.to_vk_code()) {
                                hook.key_action(KeyAction::Replace);
                            } else {
                                hook.key_action(KeyAction::Block);
                            }
                            let result = hotkey.callback();
                            if let Some(callback_result_channel) = &self.callback_results_channel {
                                callback_result_channel.send(result).unwrap();
                            }
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    hook.key_action(KeyAction::Allow);
                }
            }
        }
        hook.exit();
    }

    /// Signals the `HotkeyManager` to interrupt its event loop.
    pub fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle {
            interrupt: Arc::clone(&self.interrupt),
        }
    }

    /// Signals the `HotkeyManager` to pause processing of hotkeys.
    pub fn pause_handle(&self) -> PauseHandle {
        PauseHandle {
            pause: Arc::clone(&self.paused),
        }
    }
}

/// A handle for signaling the `HotkeyManager` to stop its event loop.
///
/// The `InterruptHandle` is used to gracefully interrupt the event loop by sending
/// a control signal. This allows the `HotkeyManager` to clean up resources and stop
/// processing keyboard events.
#[derive(Debug, Clone)]
pub struct InterruptHandle {
    interrupt: Arc<AtomicBool>,
}

impl InterruptHandle {
    /// Interrupts the `HotkeyManager`'s event loop.
    ///
    /// This method sets an internal flag to indicate that the interruption has been requested.
    /// then sends a dummy keyboard event to the event loop to force it to check the flag.
    pub fn interrupt(&self) {
        use crate::hook::{KeyboardEvent, HOOK_EVENT_TX};
        let dummy_event = KeyboardEvent::KeyDown {
            vk_code: 0,
            keyboard_state: KeyboardState::new(),
        };
        self.interrupt.store(true, Ordering::Relaxed);
        let event_tx = HOOK_EVENT_TX.read().unwrap();
        if let Some(ke_tx) = &*event_tx {
            ke_tx.send(dummy_event).unwrap();
        }
    }
}

/// A handle for signaling the `HotkeyManager` to stop processing hotkeys without
/// exiting the event loop or unregistering hotkeys. When paused, the `HotkeyManager`
/// will only process registered pause hotkeys.
///
/// The `PauseHandle` is used to manage the pause state of the `HotkeyManager`.
#[derive(Debug, Clone)]
pub struct PauseHandle {
    pause: Arc<AtomicBool>,
}

impl PauseHandle {
    /// Toggles the pause state of the `HotkeyManager`.
    ///
    /// If the `HotkeyManager` is currently paused, calling this method will resume
    /// normal hotkey processing. If it is active, calling this method will pause it.
    pub fn toggle(&self) {
        self.pause
            .store(!self.pause.load(Ordering::Relaxed), Ordering::Relaxed);
    }

    /// Explicitly sets the pause state.
    pub fn set(&self, state: bool) {
        self.pause.store(state, Ordering::Relaxed);
    }

    /// Returns whether the `HotkeyManager` is currently paused.
    ///
    /// When paused, only pause hotkeys will be processed while all others will
    /// be ignored.
    pub fn is_paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }
}
