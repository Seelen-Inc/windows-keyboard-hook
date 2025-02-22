use std::thread::{sleep, spawn};
use std::time::Duration;
use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;

fn main() { 
    // Create the manager
    let mut hkm = HotkeyManager::new();

    // Register a system-wide hotkey with trigger key 'A' and modifier key 'CTRL'
    hkm.register_hotkey(VKey::A, &[VKey::Control], || {
        println!("Hotkey CTRL + A was pressed");
    })
    .unwrap();

    // Get an interrupt handle that can be used to interrupt / stop the event loop from any thread
    let interrupt_handle = hkm.interrupt_handle();

    // Get a pause handle that can be used to programmatically pause the processing of hotkeys from
    // any thread. Note: registered pause hotkeys will still be processed.
    let pause_handle = hkm.pause_handle();

    // Create a second thread that will pause/interrupt hkm
    spawn(move || {
        sleep(Duration::from_secs(3));
        
        println!("Pausing hotkeys for 3 seconds");
        pause_handle.toggle();
        sleep(Duration::from_secs(3));

        println!("Unpausing hotkeys");
        pause_handle.toggle();

        sleep(Duration::from_secs(3));
        interrupt_handle.interrupt();
    });

    // Run the event handler in a blocking loop. This will block until interrupted and execute the
    // set callbacks when registered hotkeys are detected
    hkm.event_loop();

    println!("Event Loop interrupted");
}
