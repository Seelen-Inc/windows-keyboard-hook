# win-hotkeys
[![Crates.io](https://img.shields.io/crates/v/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![License](https://img.shields.io/crates/l/win-hotkeys.svg)](https://crates.io/crates/win-hotkeys)
[![Documentation](https://docs.rs/win-hotkeys/badge.svg)](https://docs.rs/win-hotkeys)
> A lightweight, thread-safe Rust library for managing system-wide hotkeys on Windows

The `win-hotkeys` crate simplifies working with the Windows API by abstracting and managing 
all interactions related to registering hotkeys and handling their events. Unlike many other 
solutions, this crate does not rely on the `RegisterHotKey` Windows function. Instead, it 
leverages a `WH_KEYBOARD_LL` hook to provide a more flexible and powerful way to monitor 
global hotkeys. This approach, allows for additional functionality (i.e. WIN key as modifier) and 
bypasses limitations of RegisterHotKey.

```toml
[dependencies]
win-hotkeys = "0.2.0"
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
## Keys
`win-hotkeys` provides `VKey` and `ModKey` enums that abstract the Windows Virtual Key (VK) codes. These keys
are used to specify the hotkeys available for registration.

```rust
fn main() {
    let vk_a1 = VKey::A;
    let vk_a2 = VKey::from_keyname("a").unwrap();
    let vk_a3 = VKey::from_keyname("A").unwrap();
    let vk_a4 = VKey::from_keyname("0x41").unwrap();
    let vk_a5 = VKey::from_vk_code(0x41);
    let vk_a6 = VKey::CustomKeyCode(0x41);

    assert_eq!(vk_a1, vk_a2);
    assert_eq!(vk_a1, vk_a3);
    assert_eq!(vk_a1, vk_a4);
    assert_eq!(vk_a1, vk_a5);
    assert_eq!(vk_a1, vk_a6);

    let mod_alt1 = ModKey::Alt;
    let mod_alt2 = ModKey::from_keyname("alt").unwrap();
    let mod_alt3 = ModKey::from_keyname("VK_LMENU").unwrap();
    let mod_alt4 = ModKey::from_keyname("OxA4").unwrap();
    let mod_alt5 = ModKey::from_vk_code(0xA4).unwrap();

    assert_eq!(mod_alt1, mod_alt2);
    assert_eq!(mod_alt1, mod_alt3);
    assert_eq!(mod_alt1, mod_alt4);
    assert_eq!(mod_alt1, mod_alt5);
}
```

For a full list of supported keys, refer to the [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)

## Examples
Up-to-date examples can always be found in the [examples directory](https://github.com/iholston/win-hotkeys/tree/main/examples)

## License

This project is licensed under the [MIT License](https://crates.io/crates/win-hotkeys).
See the [`LICENSE`](./LICENSE) file for details
