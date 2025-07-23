use crate::state::KeyboardState;

/// Enum representing how to handle keypress.
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

/// Enum representing keyboard input events.
///
/// **note**: This doesn't represent the real hardware event, as hooks on high priority
/// can override the pressed keys.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardInputEvent {
    KeyDown {
        /// The virtual key code of the key.
        vk_code: u16,
        /// The updated keyboard state due to this event.
        keyboard_state: KeyboardState,
    },
    KeyUp {
        /// The virtual key code of the key.
        key_code: u16,
        /// The updated keyboard state due to this event.
        keyboard_state: KeyboardState,
    },
}
