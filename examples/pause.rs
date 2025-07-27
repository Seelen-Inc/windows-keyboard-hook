use win_hotkeys::{Hotkey, HotkeyManager, HotkeysPauseHandler, VKey};

fn main() {
    let hkm = HotkeyManager::current();

    // Register a system-wide hotkey with the trigger key 'A' and the modifier key 'CTRL'
    let trigger_key = VKey::A;
    let mod_key = VKey::Control;

    let ctrl_a = Hotkey::new(trigger_key, [mod_key], || {
        println!("Hotkey CTRL + A was pressed");
    });

    let ctrl_v = Hotkey::new(VKey::V, [VKey::Control], || {
        println!("Hotkey CTRL + V was pressed");
    });

    hkm.register_hotkey(ctrl_a).unwrap();
    hkm.register_hotkey(ctrl_v).unwrap();

    // Register pause hotkey. This hotkey will toggle the pause state of win-hotkeys,
    let pause_shortcut = Hotkey::new(VKey::P, [VKey::Control, VKey::Shift], || {
        println!("Hotkey CTRL + Shift + P toggles pause state for win-hotkeys!");
        HotkeysPauseHandler::current().toggle();
    })
    .bypass_pause(); // This hotkey will not be blocked by the pause state

    hkm.register_hotkey(pause_shortcut).unwrap();

    let event_loop_thread = HotkeyManager::start_keyboard_capturing().unwrap();
    event_loop_thread.join().unwrap(); // Block until the event loop thread exits
}
