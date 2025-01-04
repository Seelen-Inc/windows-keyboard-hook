//! Defines the error types used throughout the crate.
//! Errors are represented by the [`WHKError`] enum, which encapsulates
//! various error scenarios such as invalid keys or failed hotkey registration.

use thiserror::Error;

/// An enumeration of errors that may occur while using the crate.
#[derive(Error, Debug)]
pub enum WHKError {
    #[error("Hotkey registration failed. Hotkey is already in use.")]
    RegistrationFailed,
    #[error("Invalid key name `{0}`")]
    InvalidKey(String),
}
