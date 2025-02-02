//! This module defines the `Hotkey` struct, which represents a keyboard hotkey.
//! A hotkey is composed of a trigger key, one or more modifier keys, and a callback function
//! that is executed when the hotkey is triggered.

use crate::state::KeyboardState;
use crate::VKey;
use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};

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
    callback: Box<dyn Fn() -> T + Send + 'static>,
}

impl<T> Hotkey<T> {
    /// Creates a new `Hotkey` instance.
    pub fn new(
        trigger_key: VKey,
        modifiers: &[VKey],
        callback: impl Fn() -> T + Send + 'static,
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
    pub fn is_trigger_state(&self, keyboard_state: KeyboardState) -> bool {
        let state = self.generate_keyboard_state();
        let mut keys = self.modifiers.clone();
        keys.push(self.trigger_key);

        // Ensure all hotkey keys are pressed
        for key in &keys {
            if !keyboard_state.is_down(key.to_vk_code()) {
                return false;
            }
        }
        // Ensure no extra modifiers are pressed
        state.is_down(VKey::Shift.to_vk_code()) == keyboard_state.is_down(VKey::Shift.to_vk_code())
            && state.is_down(VKey::Control.to_vk_code())
                == keyboard_state.is_down(VKey::Control.to_vk_code())
            && state.is_down(VKey::Menu.to_vk_code())
                == keyboard_state.is_down(VKey::Menu.to_vk_code())
            && state.is_down(VKey::LWin.to_vk_code())
                == keyboard_state.is_down(VKey::LWin.to_vk_code())
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
    pub fn generate_keyboard_state(&self) -> KeyboardState {
        let mut keyboard_state = KeyboardState::new();
        keyboard_state.keydown(self.trigger_key.to_vk_code());
        for key in &self.modifiers {
            keyboard_state.keydown(key.to_vk_code());
        }
        keyboard_state
    }
}

impl fmt::Debug for Hotkey<()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hotkey")
            .field("trigger_key", &self.trigger_key)
            .field("modifiers", &self.modifiers)
            .field("callback", &"<callback>")
            .finish()
    }
}

impl PartialEq for Hotkey<()> {
    fn eq(&self, other: &Self) -> bool {
        self.generate_keyboard_state() == other.generate_keyboard_state()
    }
}

impl Eq for Hotkey<()> {}
