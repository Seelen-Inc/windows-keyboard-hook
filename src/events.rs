use std::sync::LazyLock;

use crossbeam_channel::{Receiver, Sender};

use crate::state::KeyboardState;

static KIE_CHANNEL: LazyLock<(Sender<KeyboardInputEvent>, Receiver<KeyboardInputEvent>)> =
    LazyLock::new(crossbeam_channel::unbounded);

static ACTION_CHANNEL: LazyLock<(Sender<KeyAction>, Receiver<KeyAction>)> =
    LazyLock::new(crossbeam_channel::unbounded);

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

impl KeyboardInputEvent {
    pub fn send(event: Self) {
        if KIE_CHANNEL.0.send(event).is_err() {
            eprintln!("Failed to send keyboard event");
        }
    }

    pub fn recv() -> Result<KeyboardInputEvent, crossbeam_channel::RecvError> {
        KIE_CHANNEL.1.recv()
    }
}

/// Enum representing how to handle keypress.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyAction {
    Allow,
    Block,
    Replace,
}

impl KeyAction {
    pub fn send(action: Self) {
        if ACTION_CHANNEL.0.send(action).is_err() {
            eprintln!("Failed to send key action");
        }
    }

    pub fn reciever() -> Receiver<KeyAction> {
        ACTION_CHANNEL.1.clone()
    }
}
