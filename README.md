# win-hotkeys
[![Crates.io](https://img.shields.io/crates/v/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![License](https://img.shields.io/crates/l/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![Documentation](https://docs.rs/win-hotkeys/badge.svg)](https://docs.rs/win-hotkeys)
> A lightweight, thread-safe Rust library for managing system-wide hotkeys on Windows

The `win-hotkeys` crate simplifies hotkey management on Windows by abstracting interactions 
with the Windows API. It leverages the `WH_KEYBOARD_LL` low-level keyboard hook to provide a 
robust and flexible solution for global hotkeys, including full support for the `WIN` key as a 
modifier.

```toml
[dependencies]
win-hotkeys = "0.4.0"
```

## Features
- **Thread Safe**: Built with safety in mind, ensuring reliable operation across multiple threads.
- **Easy Hotkey Management**: Simple process of creating, registering, and managing hotkeys.
- **Flexible Key Combinations**: Register any set of keys (single or multiple) as a hotkey including the `WIN` key.
- **Rust Callbacks and Closures**: Assign Rust functions or closures to run when a hotkey is triggered.
- **Human-Readable Key Names**: Create `VKey` instances from intuitive string representations.
- **Efficient Performance**: Optimized to handle hotkey events with minimal overhead.

## Usage
```rust
use win_hotkeys::keys::VKey;
use win_hotkeys::HotkeyManager;

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register_hotkey(VKey::A, &[VKey::from_keyname("ctrl").unwrap()], || {
        println!("Hotkey CTRL + A was pressed");
    }).unwrap();

    hkm.register_hotkey(VKey::B, &[VKey::LWin, VKey::Shift], || {
        println!("Hotkey WIN + SHIFT + B was pressed");
    }).unwrap();

    hkm.event_loop();
}
```

## Virtual Keys 
The `VKey` enum in this library is based on the [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes), which define
key codes for a wide range of keyboard keys. This provides a comprehensive and standardized set of keys
that can be used to define hotkeys.

A small list of **common alias names** are also supported:

- `Ctrl`: `Control`(`VK_CONTROL`)
- `LCtrl` : `LControl`(`VK_LCONTROL`)
- `RCtrl`: `RControl`(`VK_RCONTROL`)
- `Alt`:  `Menu`(`VK_MENU`)
- `LAlt`: `LMenu`(`VK_LMENU`)
- `RAlt`: `RMenu`(`VK_RMENU`)
- `Win`: `LWin`(`VK_LWIN`)

For keys that have distinct left and right versions, the default key will work with either key **when used 
as a modifier**. For example, using `Shift` as a modifier will trigger the hotkey when either `LShift` or 
`RShift` is pressed, but if used as a trigger key it will only respond if `Shift` is pressed (which is not
present on most keyboards).

```rust
let key1 = VKey::from_keyname("VK_MENU").unwrap(); // Official name
let key2 = VKey::from_keyname("alt").unwrap();     // Alias for VK_MENU
let key3 = VKey::from_keyname("MENU").unwrap();    // Omitted VK_

assert_eq!(key1, key2);
assert_eq!(key1, key3);
```

## Examples
Up-to-date examples can always be found in the [examples directory](https://github.com/iholston/win-hotkeys/tree/main/examples)

## License

This project is licensed under the [MIT License](https://crates.io/crates/win-hotkeys).
See the [`LICENSE`](./LICENSE) file for details
