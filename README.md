# Win Hotkeys
[![Crates.io](https://img.shields.io/crates/v/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![License](https://img.shields.io/crates/l/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![Documentation](https://docs.rs/win-hotkeys/badge.svg)](https://docs.rs/win-hotkeys)
> A lightweight, thread-safe Rust library for managing system-wide hotkeys on Windows

The `win-hotkeys` crate simplifies working with the Windows API by abstracting and managing all interactions
related to registering hotkeys and handling their events. It provides a thread-safe access and a user-
friendly interface for managing global hotkeys.

```toml
[dependencies]
win-hotkeys = "0.1.0"
```

## How to use
```rust
use win_hotkeys::keys::{ModKey, VKey};
use win_hotkeys::HotkeyManager;

fn main() {
    let mut manager = HotkeyManager::new();

    manager.register_hotkey(VKey::A, &[ModKey::Ctrl], || {
        println!("Hotkey CTRL + A was pressed");
    }).unwrap();

    hkm.event_loop();
}
```

## Examples
Up-to-date examples can always be found [here](win-hotkeys/examples)


