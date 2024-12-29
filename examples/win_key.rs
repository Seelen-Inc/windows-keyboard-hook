use win_hotkeys::keys::{ModKey, VKey};
use win_hotkeys::HotkeyManager;

fn main() {
    // Create the manager
    let mut manager = HotkeyManager::new();

    // Registering only the WIN key effectively disables it. Preventing all default
    // OS windows shortcuts (Except WIN + L. See WIN + L below)
    //
    // NOTE: This does not prevent the WIN key from being used as a modifier in other
    // hotkeys.
    //
    manager
        .register_hotkey(VKey::LWin, &[], || {
            println!("WIN key is blocked, but still works for registered hotkeys");
        })
        .unwrap();

    // Register a system-wide hotkey with the main key 'A' and the modifier key 'WIN'
    manager
        .register_hotkey(VKey::A, &[ModKey::Win], || {
            println!("Hotkey WIN + A was pressed");
        })
        .unwrap();

    // Register a system-wide hotkey with the main key 'L' and the modifier key 'WIN'
    // The hotkey will run, however this will also cause your screen to lock. This is
    // a Windows feature and cannot be disabled via a keyboardhook.
    // Follow the steps in the following link to update the registry to disable the
    // behavior: https://superuser.com/questions/1059511/how-to-disable-winl-in-windows-10
    manager
        .register_hotkey(VKey::L, &[ModKey::Win], || {
            println!("Hotkey WIN + L was pressed, but your screen probably locked. You can disable locking in the Windows Registry.");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'A' and moidifer keys 'WIN' and 'SHIFT'
    manager
        .register_hotkey(VKey::A, &[ModKey::Win, ModKey::Shift], || {
            println!("Hotkey WIN + SHIFT + A was pressed");
        })
        .unwrap();

    manager.event_loop();
}
