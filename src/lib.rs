//! # Win-Hotkeys
//!
//! Win-hotkeys is a Rust library for creating and managing global hotkeys on Windows.
//! It provides an ergonomic API for setting up keyboard hooks, registering hotkeys,
//! and handling keyboard events in a safe and efficient manner.
#![cfg(windows)]

mod client_executor;
pub mod error;
pub mod events;
pub mod hook;
mod hotkey;
mod keys;
mod manager;
pub mod state;
mod utils;

pub use hotkey::*;
pub use keys::*;
pub use manager::*;
