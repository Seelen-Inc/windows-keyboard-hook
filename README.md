# win-hotkeys
[![Crates.io](https://img.shields.io/crates/v/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![License](https://img.shields.io/crates/l/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![Documentation](https://docs.rs/win-hotkeys/badge.svg)](https://docs.rs/win-hotkeys)
> A lightweight, thread-safe Rust library for managing system-wide hotkeys on Windows

The `win-hotkeys` crate simplifies working with the Windows API by abstracting and managing 
all interactions related to registering hotkeys and handling their events. Unlike many other 
solutions, this crate does not rely on the `RegisterHotKey` Windows function. Instead, it 
leverages a low-level keyboard hook to provide a more flexible and powerful way to monitor 
global hotkeys. This approach, allows for additional functionality (i.e. WIN key as modifier) and 
bypasses some of the limitations of RegisterHotKey.

---

```toml
[dependencies]
win-hotkeys = "0.1.0"
```

## Usage
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

## License

This project is licensed under the [MIT License](https://crates.io/crates/win-hotkeys).
See the [`LICENSE`](./LICENSE) file for details
