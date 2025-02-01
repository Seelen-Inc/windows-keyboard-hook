//! This module provides the `KeyboardState` struct to track the state of keyboard keys.
//! It supports key press (`keydown`), key release (`keyup`), and querying key state (`is_down`).

use crate::VKey;

/// Represents the state of keyboard keys.
///
/// Tracks which keys are currently pressed using two 128-bit flags, allowing
/// support for 256 keys.
#[derive(Debug, Copy, Clone)]
pub struct KeyboardState {
    pub flags: [u128; 2],
}

impl KeyboardState {
    /// Creates a new `KeyboardState` with all keys released.
    pub fn new() -> KeyboardState {
        KeyboardState { flags: [0, 0] }
    }

    /// Marks a key as pressed.
    ///
    /// Keys VK_Shift, VK_Control, and VK_Menu will be marked
    /// as pressed when either the left or right version is
    /// pressed.
    pub fn keydown(&mut self, key: u16) {
        let index = (key / 128) as usize;
        let position = key % 128;
        self.flags[index] |= 1 << position;
        match key {
            _ if key == VKey::LShift.to_vk_code() => self.keydown(VKey::Shift.to_vk_code()),
            _ if key == VKey::RShift.to_vk_code() => self.keydown(VKey::Shift.to_vk_code()),
            _ if key == VKey::LControl.to_vk_code() => self.keydown(VKey::Control.to_vk_code()),
            _ if key == VKey::RControl.to_vk_code() => self.keydown(VKey::Control.to_vk_code()),
            _ if key == VKey::LMenu.to_vk_code() => self.keydown(VKey::Menu.to_vk_code()),
            _ if key == VKey::RMenu.to_vk_code() => self.keydown(VKey::Menu.to_vk_code()),
            _ => {}
        }
    }

    /// Marks a key as released.
    pub fn keyup(&mut self, key: u16) {
        let index = (key / 128) as usize;
        let position = key % 128;
        self.flags[index] &= !(1 << position);

        if key == VKey::LShift.to_vk_code() || key == VKey::RShift.to_vk_code() {
            if !self.is_down(VKey::LShift.to_vk_code()) && !self.is_down(VKey::RShift.to_vk_code())
            {
                self.keyup(VKey::Shift.to_vk_code());
            }
        } else if key == VKey::LControl.to_vk_code() || key == VKey::RControl.to_vk_code() {
            if !self.is_down(VKey::LControl.to_vk_code())
                && !self.is_down(VKey::RControl.to_vk_code())
            {
                self.keyup(VKey::Control.to_vk_code());
            }
        } else if key == VKey::LMenu.to_vk_code() || key == VKey::RMenu.to_vk_code() {
            if !self.is_down(VKey::LMenu.to_vk_code()) && !self.is_down(VKey::RMenu.to_vk_code()) {
                self.keyup(VKey::Menu.to_vk_code());
            }
        }
    }

    /// Checks if a key is currently pressed.
    pub fn is_down(&self, key: u16) -> bool {
        let index = (key / 128) as usize;
        let position = key % 128;
        (self.flags[index] & (1 << position)) != 0
    }

    /// Clears the state of all keys, marking them as released.
    pub fn clear(&mut self) {
        self.flags = [0, 0];
    }

    /// Prints all pressed keys
    pub fn print_pressed_keys(&self) {
        print!("Pressed keys: ");
        for (i, &mask) in self.flags.iter().enumerate() {
            for bit in 0..128 {
                //match bit {
                //    _ if bit as u16 == VKey::Shift.to_vk_code() => continue,
                //    _ if bit as u16 == VKey::Control.to_vk_code() => continue,
                //    _ if bit as u16 == VKey::Menu.to_vk_code() => continue,
                //    _ => {}
                //}
                if mask & (1 << bit) != 0 {
                    let vk_code = (i * 128 + bit) as u16;
                    let vkey = VKey::from_vk_code(vk_code);
                    print!(
                        "{} ({}), ",
                        vkey.to_string().trim_start_matches("VK_"),
                        vk_code
                    );
                }
            }
        }
        println!();
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for KeyboardState {
    fn eq(&self, other: &KeyboardState) -> bool {
        self.flags == other.flags
    }
}

impl Eq for KeyboardState {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_keyboard_state() {
        let keyboard = KeyboardState::new();
        assert_eq!(
            keyboard.flags,
            [0, 0],
            "New KeyboardState should have all flags cleared"
        );
    }

    #[test]
    fn test_keydown() {
        let mut keyboard = KeyboardState::new();
        keyboard.keydown(65);
        assert_eq!(keyboard.flags[0], 1 << (65 % 128), "Key 65 should be set");

        keyboard.keydown(129);
        assert_eq!(keyboard.flags[1], 1 << (129 % 128), "Key 129 should be set");
    }

    #[test]
    fn test_keyup() {
        let mut keyboard = KeyboardState::new();
        keyboard.keydown(65); // Press key 65
        keyboard.keyup(65); // Release key 65
        assert_eq!(keyboard.flags[0], 0, "Key 65 should be cleared");

        keyboard.keydown(129); // Press key 129
        keyboard.keyup(129); // Release key 129
        assert_eq!(keyboard.flags[1], 0, "Key 129 should be cleared");
    }

    #[test]
    fn test_clear() {
        let mut keyboard = KeyboardState::new();
        keyboard.keydown(65); // Press key 65
        keyboard.keydown(129); // Press key 129
        keyboard.clear(); // Clear all flags
        assert_eq!(
            keyboard.flags,
            [0, 0],
            "KeyboardState should be cleared after clear()"
        );
    }

    #[test]
    fn test_clone() {
        let mut keyboard = KeyboardState::new();
        keyboard.keydown(65); // Press key 65
        let cloned_keyboard = keyboard.clone();
        assert_eq!(
            keyboard, cloned_keyboard,
            "Cloned KeyboardState should be equal to the original"
        );

        // Modify the original and ensure the clone is unaffected
        keyboard.keydown(129);
        assert_ne!(
            keyboard, cloned_keyboard,
            "Cloned KeyboardState should not reflect changes to the original"
        );
    }

    #[test]
    fn test_equality() {
        let mut keyboard1 = KeyboardState::new();
        let mut keyboard2 = KeyboardState::new();

        // Both are empty and should be equal
        assert_eq!(
            keyboard1, keyboard2,
            "Two empty KeyboardState instances should be equal"
        );

        // Modify one and ensure inequality
        keyboard1.keydown(65);
        assert_ne!(
            keyboard1, keyboard2,
            "KeyboardState instances with different flags should not be equal"
        );

        // Make them equal again
        keyboard2.keydown(65);
        assert_eq!(
            keyboard1, keyboard2,
            "KeyboardState instances with the same flags should be equal"
        );
    }

    #[test]
    fn test_multiple_keys() {
        let mut keyboard = KeyboardState::new();

        // Press multiple keys
        keyboard.keydown(65);
        keyboard.keydown(70);
        keyboard.keydown(129);

        assert_eq!(true, keyboard.is_down(65), "Key 65 should be set");
        assert_eq!(true, keyboard.is_down(70), "Key 70 should be set");
        assert_eq!(true, keyboard.is_down(129), "Key 129 should be set");

        // Release some keys
        keyboard.keyup(65);
        keyboard.keyup(70);

        assert_eq!(keyboard.flags[0], 0, "Key 65 should be cleared");
        assert_eq!(keyboard.flags[0], 0, "Key 70 should be cleared");
        assert_eq!(
            keyboard.flags[1],
            1 << (129 % 128),
            "Key 129 should remain set"
        );
    }
}
