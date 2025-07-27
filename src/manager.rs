//! Defines the `HotkeyManager`, which manages the registration,
//! unregistration, and execution of hotkeys. It also handles the main event
//! loop that listens for keyboard events and invokes associated callbacks.

use crate::error::Result;
use crate::error::WHKError::HotKeyAlreadyRegistered;
use crate::events::{EventLoopEvent, KeyAction, KeyboardInputEvent};
use crate::hotkey::{Hotkey, TriggerBehavior};
use crate::state::KEYBOARD_STATE;
use crate::VKey;
use crate::{hook, log_on_dev};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

type HotkeysMap = Arc<Mutex<HashMap<VKey, HashSet<Hotkey>>>>;

static HOTKEYS: LazyLock<HotkeysMap> =
    LazyLock::new(|| Arc::new(Mutex::new(HotkeyManager::get_initial_hotkeys())));
static PAUSED: AtomicBool = AtomicBool::new(false);

/// Manages the hotkeys, including their registration, unregistration, and execution.
///
/// The `HotkeyManager` listens for keyboard events and triggers the corresponding
/// hotkey callbacks when events match registered hotkeys.
#[derive(Debug)]
#[allow(dead_code)]
pub struct HotkeyManager {
    /// stores the registered hotkeys
    hotkeys: HotkeysMap,
    /// indicates whether the hotkey manager is paused
    paused: &'static AtomicBool,
}

impl HotkeyManager {
    pub fn current() -> HotkeyManager {
        HotkeyManager {
            hotkeys: HOTKEYS.clone(),
            paused: &PAUSED,
        }
    }

    /// Registers a new hotkey.
    pub fn register_hotkey(&self, hotkey: Hotkey) -> Result<u64> {
        let id = hotkey.as_hash();
        let was_already_inserted = !self
            .hotkeys
            .lock()?
            .entry(hotkey.trigger_key)
            .or_default()
            .insert(hotkey);
        if was_already_inserted {
            return Err(HotKeyAlreadyRegistered);
        }
        Ok(id)
    }

    /// Unregisters a hotkey by its unique id.
    pub fn unregister_hotkey(&self, hotkey_id: u64) -> Result<()> {
        for hotkeys in self.hotkeys.lock()?.values_mut() {
            hotkeys.retain(|hotkey| hotkey.as_hash() != hotkey_id);
        }
        Ok(())
    }

    /// Unregisters all hotkeys.
    pub fn unregister_all(&mut self) -> Result<()> {
        self.hotkeys.lock()?.clear();
        Ok(())
    }

    /// Runs the main event loop to listen for keyboard events in a separate thread.
    ///
    /// It matches events against registered hotkeys and executes the corresponding callbacks.
    pub fn start_keyboard_capturing() -> Result<std::thread::JoinHandle<()>> {
        hook::start()?;
        let handle = std::thread::spawn(|| {
            let paused_state = HotkeysPauseHandler::current();
            // clean event loop channel, to remove events before start
            while EventLoopEvent::reciever().try_recv().is_ok() {}

            'event_loop: while let Ok(loop_event) = EventLoopEvent::reciever().recv() {
                let event = match loop_event {
                    EventLoopEvent::Stop => break 'event_loop,
                    EventLoopEvent::Keyboard(event) => event,
                };

                let KeyboardInputEvent::KeyDown {
                    vk_code,
                    keyboard_state: state,
                } = event
                else {
                    continue;
                };

                if let Some(hotkeys) = HOTKEYS.lock().unwrap().get(&VKey::from(vk_code)) {
                    'search: for hotkey in hotkeys {
                        if paused_state.is_paused() && !hotkey.bypass_pause {
                            continue 'search;
                        }

                        if !hotkey.is_trigger_state(&state) {
                            continue 'search;
                        }

                        match hotkey.behaviour {
                            TriggerBehavior::PassThrough => {
                                KeyAction::send(KeyAction::Allow);
                            }
                            TriggerBehavior::StopPropagation => {
                                if state.is_down(VKey::LWin) {
                                    KeyAction::send(KeyAction::Replace);
                                } else {
                                    KeyAction::send(KeyAction::Block);
                                }
                            }
                        }

                        hotkey.execute();
                        continue 'event_loop;
                    }
                }

                // no hotkey matched, allow pass through
                KeyAction::send(KeyAction::Allow);
            }
        });
        Ok(handle)
    }

    /// This gracefully interrupt the event loop by sending
    /// a control signal. This allows the `HotkeyManager` to clean up resources and stop
    /// processing keyboard events.
    pub fn stop_keyboard_capturing() {
        EventLoopEvent::send(EventLoopEvent::Stop);
        hook::stop();
    }

    /// Signals the `HotkeyManager` to pause processing of hotkeys.
    pub fn pause_handler(&self) -> HotkeysPauseHandler {
        HotkeysPauseHandler { state: self.paused }
    }
}

/// A handle for signaling the `HotkeyManager` to stop processing hotkeys without
/// exiting the event loop or unregistering hotkeys. When paused, the `HotkeyManager`
/// will only process registered pause hotkeys.
///
/// The `PauseHandle` is used to manage the pause state of the `HotkeyManager`.
pub struct HotkeysPauseHandler {
    state: &'static AtomicBool,
}

impl HotkeysPauseHandler {
    /// Creates a new `PauseHandler` that controls the pause state of the `HotkeyManager`.
    pub fn current() -> Self {
        Self { state: &PAUSED }
    }

    /// Toggles the pause state of the `HotkeyManager`.
    ///
    /// If the `HotkeyManager` is currently paused, calling this method will resume
    /// normal hotkey processing. If it is active, calling this method will pause it.
    pub fn toggle(&self) {
        self.state
            .store(!self.state.load(Ordering::Relaxed), Ordering::Relaxed);
    }

    /// Explicitly sets the pause state.
    pub fn set(&self, state: bool) {
        self.state.store(state, Ordering::Relaxed);
    }

    /// Returns whether the `HotkeyManager` is currently paused.
    ///
    /// When paused, only pause hotkeys will be processed while all others will
    /// be ignored.
    pub fn is_paused(&self) -> bool {
        self.state.load(Ordering::Relaxed)
    }
}

impl HotkeyManager {
    fn get_initial_hotkeys() -> HashMap<VKey, HashSet<Hotkey>> {
        let lock_screen_shortcut = Hotkey::new(VKey::L, [VKey::LWin], || {
            log_on_dev!("Locking screen");
            KEYBOARD_STATE.lock().unwrap().request_syncronization();
        })
        .bypass_pause()
        .behavior(TriggerBehavior::PassThrough);

        let security_screen_shortcut =
            Hotkey::new(VKey::Delete, [VKey::Control, VKey::Menu], || {
                log_on_dev!("Security screen");
                KEYBOARD_STATE.lock().unwrap().request_syncronization();
            })
            .bypass_pause()
            .behavior(TriggerBehavior::PassThrough);

        let mut hotkeys = HashMap::new();
        hotkeys.insert(VKey::L, HashSet::from([lock_screen_shortcut]));
        hotkeys.insert(VKey::Delete, HashSet::from([security_screen_shortcut]));
        hotkeys
    }
}
