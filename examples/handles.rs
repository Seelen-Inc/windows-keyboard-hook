use std::thread::{sleep, spawn};
use std::time::Duration;
use win_hotkeys::{Hotkey, HotkeyManager, VKey};

fn main() {
    // Create the manager
    let hkm = HotkeyManager::current();

    // Register a system-wide hotkey with trigger key 'A' and modifier key 'CTRL'
    hkm.register_hotkey(Hotkey::new(VKey::A, [VKey::Control], || {
        println!("Hotkey CTRL + A was pressed");
    }))
    .unwrap();

    // Get a pause handle that can be used to programmatically pause the processing of hotkeys from
    // any thread. Note: registered pause hotkeys will still be processed.
    let pause_handler = hkm.pause_handler();

    // Create a second thread that will pause/interrupt hkm
    spawn(move || {
        sleep(Duration::from_secs(3));

        println!("Pausing hotkeys for 3 seconds");
        pause_handler.toggle();
        sleep(Duration::from_secs(3));

        println!("Unpausing hotkeys");
        pause_handler.toggle();

        sleep(Duration::from_secs(3));
        HotkeyManager::stop_keyboard_capturing();
    });

    // Run the event handler in a blocking loop. This will block until interrupted and execute the
    // set callbacks when registered hotkeys are detected
    let event_loop_thread = HotkeyManager::start_keyboard_capturing().unwrap();
    event_loop_thread.join().unwrap(); // Block until the event loop thread exits

    println!("Event Loop interrupted");
}
