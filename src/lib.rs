//! # Win-Hotkeys
//!
//! Win-hotkeys is a Rust library for creating and managing global hotkeys on Windows.
//! It provides an ergonomic API for setting up keyboard hooks, registering hotkeys,
//! and handling keyboard events in a safe and efficient manner.

#[cfg(windows)]
pub mod error;
#[cfg(windows)]
pub mod hook;
#[cfg(windows)]
pub mod hotkey;
#[cfg(windows)]
mod keys;
#[cfg(windows)]
mod manager;
#[cfg(windows)]
pub mod state;

#[cfg(windows)]
pub use keys::*;
#[cfg(windows)]
pub use manager::*;
