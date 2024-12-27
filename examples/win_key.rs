use win_hotkeys::keys::{ModKey, VKey};
use win_hotkeys::HotkeyManager;

fn main() {
    // Create the manager
    let mut manager = HotkeyManager::new();

    // Registering the WIN key will prevent the OS from receiving the keypress and
    // will prevent all WIN key OS shortcuts.
    //
    // NOTE: This does not prevent the WIN key from being used as a modifier in other
    // hotkeys.
    //
    let hotkey_id = manager
        .register_hotkey(VKey::LWin, &[], || {
            println!("WIN key is blocked, but still works for registered hotkeys");
        })
        .unwrap();
    // Comment out this line to fix the issue below
    manager.unregister_hotkey(hotkey_id);

    // Register a system-wide hotkey with the main key 'A' and the modifier key 'WIN'
    //
    // NOTE: when using the WIN key as the sole modifier key, if the WIN key is released
    // after the trigger key is pressed but before the trigger key is released. The Windows
    // search menu will still appear (the OS sees this as a single WIN key press).
    // Recommend either disabling the WIN key or not using the WIN modifier as the sole
    // modifier to prevent this issue.
    manager
        .register_hotkey(VKey::A, &[ModKey::Win], || {
            println!("Hotkey WIN + A was pressed. Try releasing the WIN key before the A key");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'A' and moidifer keys 'WIN' and 'SHIFT'
    // The above issue does not happen in this case
    manager
        .register_hotkey(VKey::A, &[ModKey::Win, ModKey::Shift], || {
            println!(
                "Hotkey WIN + SHIFT + A was pressed. The Windows search menu will never appear."
            );
        })
        .unwrap();

    manager.event_loop();
}
