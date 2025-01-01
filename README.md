# win-hotkeys
[![Crates.io](https://img.shields.io/crates/v/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![License](https://img.shields.io/crates/l/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![Documentation](https://docs.rs/win-hotkeys/badge.svg)](https://docs.rs/win-hotkeys)
> A lightweight, thread-safe Rust library for managing system-wide hotkeys on Windows

The `win-hotkeys` crate simplifies working with the Windows API by abstracting and managing 
all interactions related to registering hotkeys and handling their events. Unlike many other 
solutions, this crate does not rely on the `RegisterHotKey` Windows function. Instead, it 
leverages a `WH_KEYBOARD_LL` hook to provide a more flexible and powerful way to monitor 
global hotkeys. This approach allows for additional functionality (i.e. `WIN` key as modifier) and 
bypasses limitations of RegisterHotKey.

```toml
[dependencies]
win-hotkeys = "0.4.0"
```

## Features
- **Thread Safe**: Built with safety in mind, ensuring reliable operation across multiple threads.
- **Easy Hotkey Management**: Simple process of creating, registering, and managing hotkeys.
- **Flexible Key Combinations**: Register any set of keys (single or multiple) as a hotkey.
- **Rust Callbacks and Closures**: Assign Rust functions or closures to run when a hotkey is triggered.
- **Human-Readable Key Names**: Create `VKey` instances from intuitive string representations.
- **Efficient Performance**: Optimized to handle hotkey events with minimal overhead.

## Usage
```rust
use win_hotkeys::keys::VKey;
use win_hotkeys::HotkeyManager;

fn main() {
    let mut manager = HotkeyManager::new();

    manager.register_hotkey(VKey::A, &[VKey::Control], || {
        println!("Hotkey CTRL + A was pressed");
    }).unwrap();

    hkm.event_loop();
}
```

## Virtual Keys
`win-hotkeys` provides a `VKey` enum that abstracts the Windows Virtual-Key (VK) codes. These keys
are used to specify the hotkeys available for registration.

For a full list of supported key names and codes, refer to the [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)

Additionally, the key names `Ctrl`, `Win`, and `Alt` can be used to specify the `VK_CONTROL`, `VK_LWIN`, `VK_MENU`
keys when creating a `VKey` from key name.

## Examples
Up-to-date examples can always be found in the [examples directory](https://github.com/iholston/win-hotkeys/tree/main/examples)

## License

This project is licensed under the [MIT License](https://crates.io/crates/win-hotkeys).
See the [`LICENSE`](./LICENSE) file for details
