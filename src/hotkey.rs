//! This module defines the `Hotkey` struct, which represents a keyboard hotkey.
//! A hotkey is composed of a trigger key, one or more modifier keys, and a callback function
//! that is executed when the hotkey is triggered.

use crate::state::{KeyboardState, KEYBOARD_STATE};
use crate::{log_on_dev, VKey};
use std::collections::HashSet;
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::LazyLock;

pub static LOCK_SCREEN_SHORTCUT: LazyLock<Hotkey<()>> = LazyLock::new(|| {
    Hotkey::new(VKey::L, &[VKey::LWin], || {
        log_on_dev!("Locking screen");
        KEYBOARD_STATE.lock().unwrap().request_syncronization();
    })
});

pub static SECURITY_SCREEN_SHORTCUT: LazyLock<Hotkey<()>> = LazyLock::new(|| {
    Hotkey::new(VKey::Delete, &[VKey::Control, VKey::Menu], || {
        log_on_dev!("Security screen");
        KEYBOARD_STATE.lock().unwrap().request_syncronization();
    })
});

/// Represents a keyboard hotkey.
///
/// A `Hotkey` includes a trigger key, a set of modifier keys, and a callback function that runs
/// when the hotkey is activated.
///
/// # Type Parameters
/// - `T`: The return type of the callback function.
pub struct Hotkey<T> {
    trigger_key: VKey,
    modifiers: Vec<VKey>,
    callback: Box<dyn Fn() -> T + Send + Sync + 'static>,
}

impl<T> Hotkey<T> {
    /// Creates a new `Hotkey` instance.
    pub fn new(
        trigger_key: VKey,
        modifiers: &[VKey],
        callback: impl Fn() -> T + Send + Sync + 'static,
    ) -> Hotkey<T> {
        Self {
            trigger_key,
            modifiers: modifiers.to_vec(),
            callback: Box::new(callback),
        }
    }

    /// Executes the callback associated with the hotkey.
    ///
    /// # Returns
    /// The result of the callback function.
    pub fn callback(&self) -> T {
        (self.callback)()
    }

    /// Checks if current keyboard state should trigger hotkey callback.
    /// This should only be called if the most recent keypress is the
    /// trigger key for the hotkey.
    pub fn is_trigger_state(&self, state: &KeyboardState) -> bool {
        // ignore trigger key validation if it is a modifier (standalone windows key as example)
        if !self.trigger_key.is_modifier_key() {
            let Some(last_pressed) = state.pressing.last() else {
                return false;
            };

            // Check if the last pressed key is the trigger key
            if *last_pressed != self.trigger_key {
                return false;
            }
        }

        let expected_state = self.generate_expected_keyboard_state();

        for key in &expected_state.pressing {
            // Ignore control, menu, shift, and windows keys
            // since they are handled separately
            if key.is_modifier_key() {
                continue;
            }

            if !state.is_down(*key) {
                return false;
            }
        }

        // Ensure no extra modifiers are pressed
        expected_state.is_win_pressed() == state.is_win_pressed()
            && expected_state.is_menu_pressed() == state.is_menu_pressed()
            && expected_state.is_shift_pressed() == state.is_shift_pressed()
            && expected_state.is_control_pressed() == state.is_control_pressed()
    }

    /// Generates a unique ID for the hotkey.
    ///
    /// The ID is computed based on the trigger key and modifiers using a hash function.
    pub fn generate_id(&self) -> i32 {
        let mut hasher = DefaultHasher::new();
        self.trigger_key.hash(&mut hasher);
        self.modifiers.hash(&mut hasher);
        let hash = hasher.finish();
        (hash & 0xFFFF_FFFF) as i32
    }

    /// Generates a `KeyboardState` representing the hotkey.
    ///
    /// This includes both the trigger key and all modifier keys.
    pub fn generate_expected_keyboard_state(&self) -> KeyboardState {
        let mut keyboard_state = KeyboardState::new();
        keyboard_state.keydown(self.trigger_key);
        for key in &self.modifiers {
            keyboard_state.keydown(*key);
        }
        keyboard_state
    }
}

impl<T> fmt::Debug for Hotkey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hotkey")
            .field("trigger_key", &self.trigger_key)
            .field("modifiers", &self.modifiers)
            .field("callback", &"<callback>")
            .finish()
    }
}

impl<T> Eq for Hotkey<T> {}
impl<T> PartialEq for Hotkey<T> {
    fn eq(&self, other: &Self) -> bool {
        // ignore modifiers order
        let a: HashSet<&VKey> = HashSet::from_iter(&self.modifiers);
        let b: HashSet<&VKey> = HashSet::from_iter(&other.modifiers);
        self.trigger_key == other.trigger_key && a == b
    }
}
