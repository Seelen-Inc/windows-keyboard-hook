//! hotkey.rs
//!
//! This module defines the `Hotkey` struct, which represents a keyboard hotkey.
//! A hotkey is composed of a trigger key, one or more modifier keys, and a callback function
//! that is executed when the hotkey is triggered.

use crate::keyboard::KeyboardState;
use crate::keys::VKey;
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
