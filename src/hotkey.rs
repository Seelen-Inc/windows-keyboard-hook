//! Defines the `Hotkey` struct, which represents a keyboard hotkey.
//! Hotkeys consist of a trigger key, modifiers, and an optional blocking behavior.
//! A callback function is executed when the hotkey is triggered.

use crate::keys::{ModKey, VKey};
use std::fmt;

/// Represents a keyboard hotkey.
pub struct Hotkey<T> {
    pub id: i32,
    pub callback: Box<dyn Fn() -> T + Send + 'static>,
}

impl<T> Hotkey<T> {
    pub fn new(
        trigger: &VKey,
        modifiers: &[ModKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Hotkey<T> {
        let id =
            Self::generate_hotkey_id(trigger.to_vk_code(), ModKey::mod_mask_from_slice(modifiers));
        Self {
            id,
            callback: Box::new(callback),
        }
    }

    /// Generates a unique hotkey ID based on the trigger key and modifiers.
    pub fn generate_hotkey_id(trigger: u16, modifiers: u16) -> i32 {
        (trigger.rotate_left(5) | modifiers) as i32
    }
}

impl fmt::Debug for Hotkey<()> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hotkey")
            .field("id", &self.id)
            .field("callback", &"<callback>")
            .finish()
    }
}
