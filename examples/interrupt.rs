use std::thread::{sleep, spawn};
use std::time::Duration;
use win_hotkeys::keys::VKey;
use win_hotkeys::HotkeyManager;

fn main() {
    // Create the manager
    let mut hkm = HotkeyManager::new();

    // Register a system-wide hotkey with trigger key 'A' and modifier key 'CTRL'
    hkm.register_hotkey(VKey::A, &[VKey::Control], || {
        println!("Hotkey CTRL + A was pressed");
    })
    .unwrap();

    // Get an interrupt handle that can be used to interrupt / stop the event loop from any thread
    let handle = hkm.interrupt_handle();

    // Create a second thread that will stop the event loop after 5 seconds
    spawn(move || {
        sleep(Duration::from_secs(5));
        handle.interrupt();
    });

    // Run the event handler in a blocking loop. This will block until interrupted and execute the
    // set callbacks when registered hotkeys are detected
    hkm.event_loop();

    println!("Event Loop interrupted");
}
