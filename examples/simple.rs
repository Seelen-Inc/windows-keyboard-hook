use win_hotkeys::keys::{ModKey, VKey};
use win_hotkeys::HotkeyManager;

fn main() {
    let mut manager = HotkeyManager::new();

    // Register a system-wide hotkey with the trigger key 'A' and the modifier key 'ALT'
    manager
        .register_hotkey(VKey::A, &[ModKey::Alt], || {
            println!("Hotkey ALT + A was pressed");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'B' and multiple modifier keys
    let trigger_key = VKey::from_keyname("b").unwrap();
    let modifiers = &[
        ModKey::from_keyname("CTRL").unwrap(),
        ModKey::from_keyname("SHIFT").unwrap(),
    ];
    manager
        .register_hotkey(trigger_key, modifiers, || {
            println!("Hotkey CTRL + SHIFT + B was pressed");
        })
        .unwrap();

    // Register a system-wide hotkey with the trigger key 'C' and multiple modifier keys
    manager
        .register_hotkey(
            VKey::CustomKeyCode(0x43),
            &[ModKey::Win, ModKey::Alt],
            || {
                println!("Hotkey WIN + ALT + C was pressed");
            },
        )
        .unwrap();

    // Register system-wide hotkey with trigger key 'D' and modifier key 'ALT'
    let hotkey_id = manager
        .register_hotkey(
            VKey::from_vk_code(0x44),
            &[ModKey::from_vk_code(0xA4).unwrap()],
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
