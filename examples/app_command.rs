use crossbeam_channel::unbounded;
use std::thread;
use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;

enum AppCommand {
    AppCommand1,
    AppCommand2,
    Exit,
}

fn main() {
    // The HotkeyManager is generic over the return type of the callback functions.
    let mut hkm = HotkeyManager::new();
    let modifiers = &[VKey::LWin, VKey::Shift];

    // Register WIN + SHIFT + 1 for app command 1
    hkm.register_hotkey(VKey::Vk1, modifiers, || {
        println!("Pressed WIN + SHIFT + 1");
        AppCommand::AppCommand1
    })
    .unwrap();

    // Register WIN + SHIFT + 2 for app command 2
    hkm.register_hotkey(VKey::Vk2, modifiers, || {
        println!("Pressed WIN + SHIFT + 2");
        AppCommand::AppCommand2
    })
    .unwrap();

    // Register WIN + 3 for app command EXIT
    hkm.register_hotkey(VKey::Vk3, modifiers, || {
        println!("Pressed WIN + SHIFT + 3");
        AppCommand::Exit
    })
    .unwrap();

    // Register channel to receive events from the hkm event loop
    let (tx, rx) = unbounded();
    hkm.register_channel(tx);

    // Run HotkeyManager in background thread
    let handle = hkm.interrupt_handle();
    thread::spawn(move || {
        hkm.event_loop();
    });

    // App Logic
    loop {
        let command = rx.recv().unwrap();

        match command {
            AppCommand::AppCommand1 => {
                println!("Do thing 1");
            }
            AppCommand::AppCommand2 => {
                println!("Do thing 2");
            }
            AppCommand::Exit => {
                println!("Exiting...");
                handle.interrupt();
                break;
            }
        }
    }
}
