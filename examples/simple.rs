use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;

fn main() {
    let mut hkm = HotkeyManager::new();

    // Register a system-wide hotkey with the trigger key 'A' and the modifier key 'ALT'
    let trigger_key = VKey::from_keyname("a").unwrap();
    let mod_key = VKey::from_keyname("alt").unwrap();
    hkm.register_hotkey(trigger_key, &[mod_key], || {
        println!("Hotkey ALT + A was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey with the trigger key 'B' and the modifier key 'f24'
    let trigger_key = VKey::from_keyname("b").unwrap();
    let modifiers = &[VKey::from_vk_code(0x87)];
    hkm.register_hotkey(trigger_key, modifiers, || {
        println!("Hotkey F24 + B was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey with the trigger key 'C' and multiple modifier key
    hkm.register_hotkey(VKey::C, &[VKey::LWin, VKey::Menu], || {
        println!("Hotkey WIN + ALT + C was pressed");
    })
    .unwrap();

    // Register and store id for system-wide hotkey with trigger key 'D' and modifier key 'ALT'
    let hotkey_id = hkm
        .register_hotkey(
            VKey::from_vk_code(0x44),
            &[VKey::from_vk_code(0xA4)],
            || {
                println!("Hotkey ALT + D was pressed");
            },
        )
        .unwrap();

    // Unregister hotkey with ID
    hkm.unregister_hotkey(hotkey_id);

    // Run the event handler in a blocking loop. This will block forever and execute the set
    // callbacks when the registered hotkeys are detected
    let _ = hkm.event_loop();
}
