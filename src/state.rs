use windows::Win32::UI::Input::KeyboardAndMouse::{VK_CONTROL, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_LWIN, VK_MENU, VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_RWIN, VK_SHIFT};

#[derive(Debug, Copy, Clone)]
pub struct KeyboardState {
    flags: [u128; 2],
}

impl KeyboardState {

    pub fn new() -> KeyboardState {
        KeyboardState { flags: [0, 0] }
    }

    pub fn keydown(&mut self, mut key: u16) {
        key = KeyboardState::convert_if_mod_key(key);
        let index = (key / 128) as usize;
        let position = key % 128;
        self.flags[index] |= 1 << position;
    }

    pub fn keyup(&mut self, mut key: u16) {
        key = KeyboardState::convert_if_mod_key(key);
        let index = (key / 128) as usize;
        let position = key % 128;
        self.flags[index] &= !(1 << position);
    }

    pub fn is_down(&self, mut key: u16) -> bool {
        key = KeyboardState::convert_if_mod_key(key);
        let index = (key / 128) as usize;
        let position = key % 128;
        (self.flags[index] & (1 << position)) != 0
    }

    pub fn clear(&mut self) {
        self.flags = [0, 0];
    }

    fn convert_if_mod_key(key: u16) -> u16 {
        match key {
            _ if key == VK_CONTROL.0 => VK_LCONTROL.0,
            _ if key == VK_LCONTROL.0 => VK_LCONTROL.0,
            _ if key == VK_RCONTROL.0 => VK_LCONTROL.0,
            _ if key == VK_SHIFT.0 => VK_LSHIFT.0,
            _ if key == VK_LSHIFT.0 => VK_LSHIFT.0,
            _ if key == VK_RSHIFT.0 => VK_LSHIFT.0,
            _ if key == VK_MENU.0 => VK_LMENU.0,
            _ if key == VK_LMENU.0 => VK_LMENU.0,
            _ if key == VK_RMENU.0 => VK_LMENU.0,
            _ if key == VK_LWIN.0 => VK_LWIN.0,
            _ if key == VK_RWIN.0 => VK_LWIN.0,
            _ => key
        }
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
        assert_eq!(keyboard.flags, [0, 0], "New KeyboardState should have all flags cleared");
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
        keyboard.keyup(65);   // Release key 65
        assert_eq!(keyboard.flags[0], 0, "Key 65 should be cleared");

        keyboard.keydown(129); // Press key 129
        keyboard.keyup(129);   // Release key 129
        assert_eq!(keyboard.flags[1], 0, "Key 129 should be cleared");
    }

    #[test]
    fn test_clear() {
        let mut keyboard = KeyboardState::new();
        keyboard.keydown(65);  // Press key 65
        keyboard.keydown(129); // Press key 129
        keyboard.clear();      // Clear all flags
        assert_eq!(keyboard.flags, [0, 0], "KeyboardState should be cleared after clear()");
    }

    #[test]
    fn test_clone() {
        let mut keyboard = KeyboardState::new();
        keyboard.keydown(65); // Press key 65
        let cloned_keyboard = keyboard.clone();
        assert_eq!(keyboard, cloned_keyboard, "Cloned KeyboardState should be equal to the original");

        // Modify the original and ensure the clone is unaffected
        keyboard.keydown(129);
        assert_ne!(keyboard, cloned_keyboard, "Cloned KeyboardState should not reflect changes to the original");
    }

    #[test]
    fn test_equality() {
        let mut keyboard1 = KeyboardState::new();
        let mut keyboard2 = KeyboardState::new();

        // Both are empty and should be equal
        assert_eq!(keyboard1, keyboard2, "Two empty KeyboardState instances should be equal");

        // Modify one and ensure inequality
        keyboard1.keydown(65);
        assert_ne!(keyboard1, keyboard2, "KeyboardState instances with different flags should not be equal");

        // Make them equal again
        keyboard2.keydown(65);
        assert_eq!(keyboard1, keyboard2, "KeyboardState instances with the same flags should be equal");
    }

    #[test]
    fn test_multiple_keys() {
        let mut keyboard = KeyboardState::new();

        // Press multiple keys
        keyboard.keydown(65);
        keyboard.keydown(70);
        keyboard.keydown(129);

        assert_eq!(keyboard.flags[0], 1 << (65 % 128), "Key 65 should be set");
        assert_eq!(keyboard.flags[0], 1 << (70 % 128), "Key 70 should be set");
        assert_eq!(keyboard.flags[1], 1 << (129 % 128), "Key 129 should be set");

        // Release some keys
        keyboard.keyup(65);
        keyboard.keyup(70);

        assert_eq!(keyboard.flags[0], 0, "Key 65 should be cleared");
        assert_eq!(keyboard.flags[0], 0, "Key 70 should be cleared");
        assert_eq!(keyboard.flags[1], 1 << (129 % 128), "Key 129 should remain set");
    }
}