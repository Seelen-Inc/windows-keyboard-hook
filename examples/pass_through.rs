use win_hotkeys::{Hotkey, HotkeyManager, TriggerBehavior, VKey};

fn main() {
    let hkm = HotkeyManager::current();

    hkm.register_hotkey(
        Hotkey::new(VKey::C, [VKey::Control], || {
            println!("Hotkey CTRL + C was triggered, but will be not blocked");
        })
        .behavior(TriggerBehavior::PassThrough),
    )
    .unwrap();

    hkm.register_hotkey(
        Hotkey::new(VKey::V, [VKey::Control], || {
            println!("Hotkey CTRL + V was triggered, but will be not blocked");
        })
        .behavior(TriggerBehavior::PassThrough),
    )
    .unwrap();

    let event_loop_thread = HotkeyManager::start_keyboard_capturing().unwrap();
    // Block until the event loop thread exits
    event_loop_thread.join().unwrap();
}
