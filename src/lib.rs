//! # Win-Hotkeys
//!
//! Win-hotkeys is a Rust library for creating and managing global hotkeys on Windows.
//! It provides an ergonomic API for setting up keyboard hooks, registering hotkeys,
//! and handling keyboard events in a safe manner.

#[cfg(windows)]
pub mod error;
#[cfg(windows)]
pub mod hook;
#[cfg(windows)]
pub mod hotkey;
#[cfg(windows)]
pub mod keys;
#[cfg(windows)]
mod manager;

#[cfg(windows)]
pub use manager::*;
