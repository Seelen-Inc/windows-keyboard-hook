use win_hotkeys::keys::VKey;
use win_hotkeys::HotkeyManager;

fn main() {
    // Create the manager
    let mut hkm = HotkeyManager::new();

    // Registering only the WIN key effectively disables it. Preventing all default
    // OS windows shortcuts (Except WIN + L. See WIN + L below)
    //
    // NOTE: This does not prevent the WIN key from being used as a modifier in other
    // hotkeys.
    //
    hkm.register_hotkey(VKey::LWin, &[], || {
        println!("WIN key is blocked, but still works for registered hotkeys");
    })
    .unwrap();

    // Register a system-wide hotkey with the main key 'A' and the modifier key 'WIN'
    //
    // NOTE: LWin and RWin are equivalent keys, it does not matter which you use
    hkm.register_hotkey(VKey::A, &[VKey::LWin], || {
        println!("Hotkey WIN + A was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey with the main key 'L' and the modifier key 'WIN'
    // The hotkey will run, however this will also cause your screen to lock. This is
    // a Windows feature and cannot be disabled via a keyboardhook.
    // Follow the steps in the following link to update the registry to disable the
    // behavior: https://superuser.com/questions/1059511/how-to-disable-winl-in-windows-10
    hkm
        .register_hotkey(VKey::L, &[VKey::from_keyname("win").unwrap()], || {
            println!("Hotkey WIN + L was pressed, but your screen probably locked. You can disable locking in the Windows Registry.");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'A' and moidifer keys 'WIN' and 'SHIFT'
    hkm.register_hotkey(VKey::A, &[VKey::LWin, VKey::Shift], || {
        println!("Hotkey WIN + SHIFT + A was pressed");
    })
    .unwrap();

    hkm.event_loop();
}
