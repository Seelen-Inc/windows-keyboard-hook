//! Defines the `Hotkey` struct, which represents a keyboard hotkey.
//! Hotkeys consist of a trigger key, modifiers, and an optional blocking behavior.
//! A callback function is executed when the hotkey is triggered.

use std::fmt;
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::state::KeyboardState;
use crate::keys::VKey;

/// Represents a keyboard hotkey.
pub struct Hotkey<T> {
    trigger_key: VKey,
    modifiers: Vec<VKey>,
    callback: Box<dyn Fn() -> T + Send + 'static>,
}

impl<T> Hotkey<T> {
    pub fn new(trigger_key: VKey, modifiers: &[VKey], callback: impl Fn() -> T + Send + 'static) -> Hotkey<T> {
        Self {
            trigger_key,
            modifiers: modifiers.to_vec(),
            callback: Box::new(callback),
        }
    }

    pub fn callback(&self) -> T {
        (self.callback)()
    }

    pub fn generate_keyboard_state(&self) -> KeyboardState {
        let mut keyboard_state = KeyboardState::new();
        keyboard_state.keydown(self.trigger_key.to_vk_code());
        for key in &self.modifiers {
            keyboard_state.keydown(key.to_vk_code());
        }
        keyboard_state
    }

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