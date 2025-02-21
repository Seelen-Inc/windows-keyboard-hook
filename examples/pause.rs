use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;

fn main() {
    let mut hkm = HotkeyManager::new();

    // Register a system-wide hotkey with the trigger key 'A' and the modifier key 'CTRL'
    let trigger_key = VKey::A;
    let mod_key = VKey::Control;
    hkm.register_hotkey(trigger_key, &[mod_key], || {
        println!("Hotkey CTRL + A was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey with the trigger key 'V' and the modifier key 'CTRL'
    hkm.register_hotkey(VKey::V, &[VKey::Control], || {
        println!("Hotkey CTRL + V was pressed");
    })
    .unwrap();

    // Register pause hotkey. This hotkey will "turn off" all other hotkeys until it is pressed again.
    let trigger_key = VKey::P;
    let modifiers = &[VKey::Control, VKey::Shift];
    hkm.register_pause_hotkey(trigger_key, modifiers, || {
        println!("Hotkey CTRL + Shift + P toggles pause state for win-hotkeys!");
    })
    .unwrap();

    hkm.event_loop();
}
