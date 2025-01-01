use win_hotkeys::keys::VKey;
use win_hotkeys::HotkeyManager;

fn main() {
    let mut manager = HotkeyManager::new();

    // Register a system-wide hotkey with the trigger key 'A' and the modifier key 'ALT'
    manager
        .register_hotkey(VKey::A, &[VKey::from_keyname("alt").unwrap()], || {
            println!("Hotkey ALT + A was pressed");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'B' and the modifier key 'f24'
    let trigger_key = VKey::from_keyname("b").unwrap();
    let modifiers = &[VKey::from_vk_code(0x87)];
    manager
        .register_hotkey(trigger_key, modifiers, || {
            println!("Hotkey F24 + B was pressed");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'C' and multiple modifier key
    manager
        .register_hotkey(
            VKey::CustomKeyCode(0x43),
            &[VKey::LWin, VKey::Menu],
            || {
                println!("Hotkey WIN + ALT + C was pressed");
            },
        )
        .unwrap();

    // Register system-wide hotkey with trigger key 'D' and modifier key 'ALT'
    let hotkey_id = manager
        .register_hotkey(
            VKey::from_vk_code(0x44),
            &[VKey::from_vk_code(0xA4)],
            || {
                println!("Hotkey ALT + D was pressed");
            },
        )
        .unwrap();

    // Unregister hotkey with ID
    manager.unregister_hotkey(hotkey_id);

    // Run the event handler in a blocking loop. This will block forever and execute the set
    // callbacks when the registered hotkeys are detected
    manager.event_loop();
}
