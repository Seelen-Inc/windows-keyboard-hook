//! Defines the error types used throughout the crate.
//! Errors are represented by the [`WHKError`] enum, which encapsulates
//! various error scenarios such as invalid keys or failed hotkey registration.

use thiserror::Error;

use crate::VKey;

/// An enumeration of errors that may occur while using the crate.
#[derive(Error, Debug)]
pub enum WHKError {
    #[error("Hotkey manager thread is already started.")]
    AlreadyStarted,
    #[error("Failed to start hook thread.")]
    StartupFailed,
    #[error("Hotkey registration failed. Hotkey is already in use.")]
    HotKeyAlreadyRegistered,
    #[error("Invalid trigger key `{0:?}`")]
    HotkeyInvalidTriggerKey(VKey),
    #[error("Invalid key name `{0}`")]
    InvalidKey(String),
    // crossbeam
    #[error("Sending event failed")]
    SendFailed,
    #[error("Receiving event failed")]
    RecvFailed(#[from] crossbeam_channel::RecvError),
    #[error("Mutext lock error (poisoned)")]
    LockError,
}

impl<T> From<crossbeam_channel::SendError<T>> for WHKError {
    fn from(_err: crossbeam_channel::SendError<T>) -> Self {
        WHKError::SendFailed
    }
}

impl<T> From<std::sync::PoisonError<T>> for WHKError {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        WHKError::LockError
    }
}

pub type Result<T, E = WHKError> = std::result::Result<T, E>;
