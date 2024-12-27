//! Defines the virtual keys (`VKey`) and modifier keys (`ModKey`) used
//! for creating and managing hotkeys. These types encapsulate the functionality of
//! [virtual-key codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes).
//!
//! ## Features
//! - `VKey`: Represents virtual keys (e.g., `A`, `F1`, `Space`, etc.).
//! - `ModKey`: Represents modifier keys (`Ctrl`, `Shift`, `Alt`, `Win`).
//! - Conversion utilities between string representations, virtual-key codes, and modifiers.
//! - Bitmasking support for combining and comparing modifier keys.
//!
//! ## See Also
//! - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
//! - [`VKey`]: Virtual key definitions and utilities.
//! - [`ModKey`]: Modifier key definitions and utilities.

mod modkey;
mod vkey;

pub use modkey::*;
pub use vkey::*;
