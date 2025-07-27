use std::sync::LazyLock;

use crossbeam_channel::{Receiver, Sender};

use crate::{log_on_dev, state::KeyboardState};

static EVENT_LOOP_CHANNEL: LazyLock<(Sender<EventLoopEvent>, Receiver<EventLoopEvent>)> =
    LazyLock::new(crossbeam_channel::unbounded);

static ACTION_CHANNEL: LazyLock<(Sender<KeyAction>, Receiver<KeyAction>)> =
    LazyLock::new(crossbeam_channel::unbounded);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventLoopEvent {
    Stop,
    Keyboard(KeyboardInputEvent),
}

impl EventLoopEvent {
    pub(crate) fn send(event: Self) {
        if EVENT_LOOP_CHANNEL.0.send(event).is_err() {
            log_on_dev!("Failed to send event");
        }
    }

    pub(crate) fn reciever() -> Receiver<EventLoopEvent> {
        EVENT_LOOP_CHANNEL.1.clone()
    }
}

/// Enum representing keyboard input events.
///
/// **note**: This doesn't represent the real hardware event, as hooks on high priority
/// can override the pressed keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardInputEvent {
    KeyDown {
        /// The virtual key code of the key.
        vk_code: u16,
        /// The updated keyboard state due to this event.
        keyboard_state: KeyboardState,
    },
    KeyUp {
        /// The virtual key code of the key.
        vk_code: u16,
        /// The updated keyboard state due to this event.
        keyboard_state: KeyboardState,
    },
}

impl KeyboardInputEvent {
    pub(crate) fn send(event: Self) {
        EventLoopEvent::send(EventLoopEvent::Keyboard(event));
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
    pub(crate) fn send(action: Self) {
        if ACTION_CHANNEL.0.send(action).is_err() {
            log_on_dev!("Failed to send key action");
        }
    }

    pub(crate) fn reciever() -> Receiver<KeyAction> {
        ACTION_CHANNEL.1.clone()
    }
}
