//! This module defines the `Hotkey` struct, which represents a keyboard hotkey.
//! A hotkey is composed of a trigger key, one or more modifier keys, and a callback function
//! that is executed when the hotkey is triggered.

use crate::state::KeyboardState;
use crate::VKey;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Defines what should happen with the key event after hotkey triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerBehavior {
    /// Allow the key event to propagate to other applications
    PassThrough,
    /// Consume the key event and prevent further processing
    StopPropagation,
}

/// Represents a keyboard shortcut that triggers an action
pub struct Hotkey {
    /// key that must be pressed to trigger this hotkey
    pub trigger_key: VKey,
    /// keys that must be pressed before the trigger key ex: [CTRL] + [A]
    pub modifiers: BTreeSet<VKey>,
    /// action to perform when this hotkey is triggered
    pub behaviour: TriggerBehavior,
    /// will ignore the `paused` global state
    pub bypass_pause: bool,
    /// callback function to execute when this hotkey is triggered
    pub callback: Box<dyn Fn() + Send + Sync + 'static>,
}

impl Hotkey {
    /// Creates a new `Hotkey` instance.
    pub fn new<M, F>(trigger_key: VKey, modifiers: M, callback: F) -> Hotkey
    where
        M: AsRef<[VKey]>,
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            trigger_key,
            behaviour: TriggerBehavior::StopPropagation,
            bypass_pause: false,
            modifiers: modifiers.as_ref().iter().cloned().collect(),
            callback: Box::new(callback),
        }
    }

    /// Sets the behavior when hotkey triggers
    pub fn behavior(mut self, action: TriggerBehavior) -> Self {
        self.behaviour = action;
        self
    }

    /// Makes the hotkey work even when global hotkeys are paused
    pub fn bypass_pause(mut self) -> Self {
        self.bypass_pause = true;
        self
    }

    /// Executes the callback associated with the hotkey.
    pub fn execute(&self) {
        (self.callback)()
    }

    /// Checks if current keyboard state should trigger hotkey callback.
    /// This should only be called if the most recent keypress is the
    /// trigger key for the hotkey.
    pub fn is_trigger_state(&self, state: &KeyboardState) -> bool {
        // For non-modifier keys, verify the last pressed key matches
        if !self.trigger_key.is_modifier_key() {
            let Some(last_pressed) = state.pressing.last() else {
                return false;
            };

            if *last_pressed != self.trigger_key {
                return false;
            }
        }

        let expected_state = self.generate_expected_keyboard_state();

        // Verify all required non-modifier keys are pressed
        for key in &expected_state.pressing {
            if !key.is_modifier_key() && !state.is_down(*key) {
                return false;
            }
        }

        // Verify modifier key states match exactly
        expected_state.is_win_pressed() == state.is_win_pressed()
            && expected_state.is_menu_pressed() == state.is_menu_pressed()
            && expected_state.is_shift_pressed() == state.is_shift_pressed()
            && expected_state.is_control_pressed() == state.is_control_pressed()
    }

    /// Generates a `KeyboardState` representing the hotkey.
    pub fn generate_expected_keyboard_state(&self) -> KeyboardState {
        let mut keyboard_state = KeyboardState::new();
        keyboard_state.keydown(self.trigger_key);
        for key in &self.modifiers {
            keyboard_state.keydown(*key);
        }
        keyboard_state
    }

    /// Returns a hash representing the hotkey combination
    pub fn as_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl fmt::Debug for Hotkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hotkey")
            .field("trigger_key", &self.trigger_key)
            .field("trigger_action", &self.behaviour)
            .field("modifiers", &self.modifiers)
            .field("callback", &"<callback>")
            .finish()
    }
}

impl Eq for Hotkey {}
impl PartialEq for Hotkey {
    fn eq(&self, other: &Self) -> bool {
        self.trigger_key == other.trigger_key && self.modifiers == other.modifiers
    }
}

impl Hash for Hotkey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trigger_key.hash(state);
        self.modifiers.hash(state);
    }
}
