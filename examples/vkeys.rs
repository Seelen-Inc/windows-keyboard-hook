use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_A;

fn main() {
    // Create HotkeyManager
    let mut hkm = HotkeyManager::<()>::new();

    let vk_a1 = VKey::A;
    let vk_a2 = VKey::from_keyname("a").unwrap();
    let vk_a3 = VKey::from_vk_code(0x41);
    let vk_a4 = VKey::from_vk_code(VK_A.0);

    // Create custom keycode equivalent to A
    let vk_a5 = VKey::CustomKeyCode(0x41);

    assert_eq!(vk_a1, vk_a2);
    assert_eq!(vk_a1, vk_a3);
    assert_eq!(vk_a1, vk_a4);
    assert_eq!(vk_a1, vk_a5);

    // NOTE
    // When matching `CustomKeyCodes` you must include a guard if statement
    match vk_a5 {
        VKey::A => {
            // This will never show up
            println!("CustomKeyCode(0x41) matches against VKey::A");
        }
        _ => {
            println!("You didn't use the match statement correctly!");
        }
    }

    // Instead match like this
    match vk_a5 {
        _ if VKey::A == vk_a5 => {
            // This will match
            println!("CustomKeyCode(0x41) matches against VKey::A");
        }
        _ => {}
    }

    hkm.register_hotkey(vk_a5, &[], || {
        println!("You pressed A");
    })
    .unwrap();

    hkm.event_loop();
}
