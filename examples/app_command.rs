use std::sync::LazyLock;

use crossbeam_channel::{Receiver, Sender};
use win_hotkeys::{Hotkey, HotkeyManager, VKey};

enum AppCommand {
    AppCommand1,
    AppCommand2,
    Exit,
}

static COMMAND_CHANNEL: LazyLock<(Sender<AppCommand>, Receiver<AppCommand>)> =
    LazyLock::new(crossbeam_channel::unbounded);

fn send_command(command: AppCommand) {
    COMMAND_CHANNEL.0.send(command).unwrap();
}

fn main() {
    // The HotkeyManager is generic over the return type of the callback functions.
    let hkm = HotkeyManager::current();

    let modifiers = &[VKey::LWin, VKey::Shift];

    // Register WIN + SHIFT + 1 for app command 1
    hkm.register_hotkey(Hotkey::new(VKey::Vk1, modifiers, || {
        println!("Pressed WIN + SHIFT + 1");
        send_command(AppCommand::AppCommand1);
    }))
    .unwrap();

    // Register WIN + SHIFT + 2 for app command 2
    hkm.register_hotkey(Hotkey::new(VKey::Vk2, modifiers, || {
        println!("Pressed WIN + SHIFT + 2");
        send_command(AppCommand::AppCommand2);
    }))
    .unwrap();

    // Register WIN + 3 for app command EXIT
    hkm.register_hotkey(
        Hotkey::new(VKey::Vk3, modifiers, || {
            println!("Pressed WIN + SHIFT + 3");
            send_command(AppCommand::Exit);
        })
        .bypass_pause(), // allow exit inclusively if hotkeys are paused
    )
    .unwrap();

    // Run HotkeyManager in background thread
    HotkeyManager::start_keyboard_capturing().unwrap();

    // App Logic
    while let Ok(command) = COMMAND_CHANNEL.1.recv() {
        match command {
            AppCommand::AppCommand1 => {
                println!("Do thing 1");
            }
            AppCommand::AppCommand2 => {
                println!("Do thing 2");
            }
            AppCommand::Exit => {
                println!("Exiting...");
                HotkeyManager::stop_keyboard_capturing();
                break;
            }
        }
    }
}
