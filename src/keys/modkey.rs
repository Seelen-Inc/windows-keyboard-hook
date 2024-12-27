use crate::error::WHKError;
use crate::keys::VKey;

/// Represents a modifier key used in hotkeys.
///
/// The available modifier keys are:
/// - `Ctrl`: The Control key.
/// - `Shift`: The Shift key.
/// - `Alt`: The Alt key.
/// - `Win`: The Windows key, also referred to as the Super key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModKey {
    Ctrl,
    Shift,
    Alt,
    Win,
}

impl ModKey {
    /// Creates a `ModKey` from a string representation.
    ///
    /// # Supported Values
    /// - "CTRL" or "CONTROL"
    /// - "SHIFT"
    /// - "ALT"
    /// - "WIN", "WINDOWS", or "SUPER"
    pub fn from_keyname(name: &str) -> Result<ModKey, WHKError> {
        Ok(match name.to_ascii_uppercase().as_ref() {
            "CTRL" | "CONTROL" => ModKey::Ctrl,
            "SHIFT" => ModKey::Shift,
            "ALT" => ModKey::Alt,
            "WIN" | "WINDOWS" | "SUPER" => ModKey::Win,
            val => return Err(WHKError::InvalidModKey(val.to_string())),
        })
    }

    /// Converts the `ModKey` to its corresponding Windows virtual key (VK) code.
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    pub fn to_vk_code(&self) -> u16 {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        match self {
            ModKey::Ctrl => VK_LCONTROL.0,
            ModKey::Shift => VK_LSHIFT.0,
            ModKey::Alt => VK_LMENU.0,
            ModKey::Win => VK_LWIN.0,
        }
    }

    /// Creates a `ModKey` from a Windows virtual key (VK) code.
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    pub fn from_vk_code(vk_code: u16) -> Result<ModKey, WHKError> {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        Ok(match vk_code {
            key if key == VK_CONTROL.0 => ModKey::Ctrl,
            key if key == VK_LCONTROL.0 => ModKey::Ctrl,
            key if key == VK_RCONTROL.0 => ModKey::Ctrl,
            key if key == VK_SHIFT.0 => ModKey::Shift,
            key if key == VK_LSHIFT.0 => ModKey::Shift,
            key if key == VK_RSHIFT.0 => ModKey::Shift,
            key if key == VK_MENU.0 => ModKey::Alt,
            key if key == VK_LMENU.0 => ModKey::Alt,
            key if key == VK_RMENU.0 => ModKey::Alt,
            key if key == VK_LWIN.0 => ModKey::Win,
            key if key == VK_RWIN.0 => ModKey::Win,
            _ => {
                return Err(WHKError::VKNotAModKey(vk_code));
            }
        })
    }

    /// Combines a slice of `ModKey` values into a bitmask.
    pub fn mod_mask_from_slice(mod_keys: &[ModKey]) -> u16 {
        let mut mask = 0x00u16;
        for mod_key in mod_keys {
            mask |= mod_key.to_mod_bit();
        }
        mask
    }

    /// Creates a bitmask from boolean values representing modifier keys.
    pub fn mod_mask_from_bool(ctrl: bool, shift: bool, alt: bool, win: bool) -> u16 {
        let mut mask = 0x00u16;
        if ctrl {
            mask |= ModKey::Ctrl.to_mod_bit();
        }
        if shift {
            mask |= ModKey::Shift.to_mod_bit();
        }
        if alt {
            mask |= ModKey::Alt.to_mod_bit();
        }
        if win {
            mask |= ModKey::Win.to_mod_bit();
        }
        mask
    }

    /// Converts the `ModKey` to its bitmask representation.
    ///
    /// Each `ModKey` contributes a unique bit to the bitmask:
    /// - Ctrl -> 0x01
    /// - Shift -> 0x02
    /// - Alt -> 0x04
    /// - Win -> 0x08
    ///
    pub fn to_mod_bit(&self) -> u16 {
        match self {
            ModKey::Ctrl => 0x01u16,
            ModKey::Shift => 0x02u16,
            ModKey::Alt => 0x04u16,
            ModKey::Win => 0x08u16,
        }
    }
}

impl std::fmt::Display for ModKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ModKey::Ctrl => "Ctrl",
            ModKey::Shift => "Shift",
            ModKey::Alt => "Alt",
            ModKey::Win => "Win",
        };
        write!(f, "{}", name)
    }
}

impl From<ModKey> for VKey {
    fn from(mk: ModKey) -> VKey {
        match mk {
            ModKey::Ctrl => VKey::Control,
            ModKey::Shift => VKey::Shift,
            ModKey::Alt => VKey::Menu,
            ModKey::Win => VKey::LWin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_keyname() {
        assert_eq!(ModKey::from_keyname("ctrl").unwrap(), ModKey::Ctrl);
        assert_eq!(ModKey::from_keyname("CONTROL").unwrap(), ModKey::Ctrl);
        assert_eq!(ModKey::from_keyname("shift").unwrap(), ModKey::Shift);
        assert_eq!(ModKey::from_keyname("ALT").unwrap(), ModKey::Alt);
        assert_eq!(ModKey::from_keyname("win").unwrap(), ModKey::Win);
        assert_eq!(ModKey::from_keyname("WINDOWS").unwrap(), ModKey::Win);
        assert_eq!(ModKey::from_keyname("SUPER").unwrap(), ModKey::Win);
        assert!(ModKey::from_keyname("invalid").is_err());
    }

    #[test]
    fn test_to_vk_code() {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        assert_eq!(ModKey::Ctrl.to_vk_code(), VK_LCONTROL.0);
        assert_eq!(ModKey::Shift.to_vk_code(), VK_LSHIFT.0);
        assert_eq!(ModKey::Alt.to_vk_code(), VK_LMENU.0);
        assert_eq!(ModKey::Win.to_vk_code(), VK_LWIN.0);
    }

    #[test]
    fn test_from_vk_code() {
        use windows::Win32::UI::Input::KeyboardAndMouse::*;
        assert_eq!(ModKey::from_vk_code(VK_CONTROL.0).unwrap(), ModKey::Ctrl);
        assert_eq!(ModKey::from_vk_code(VK_LCONTROL.0).unwrap(), ModKey::Ctrl);
        assert_eq!(ModKey::from_vk_code(VK_RCONTROL.0).unwrap(), ModKey::Ctrl);
        assert_eq!(ModKey::from_vk_code(VK_SHIFT.0).unwrap(), ModKey::Shift);
        assert_eq!(ModKey::from_vk_code(VK_LSHIFT.0).unwrap(), ModKey::Shift);
        assert_eq!(ModKey::from_vk_code(VK_RSHIFT.0).unwrap(), ModKey::Shift);
        assert_eq!(ModKey::from_vk_code(VK_MENU.0).unwrap(), ModKey::Alt);
        assert_eq!(ModKey::from_vk_code(VK_LMENU.0).unwrap(), ModKey::Alt);
        assert_eq!(ModKey::from_vk_code(VK_RMENU.0).unwrap(), ModKey::Alt);
        assert_eq!(ModKey::from_vk_code(VK_LWIN.0).unwrap(), ModKey::Win);
        assert_eq!(ModKey::from_vk_code(VK_RWIN.0).unwrap(), ModKey::Win);
        assert!(ModKey::from_vk_code(0x00).is_err());
    }

    #[test]
    fn test_mod_mask_from_slice() {
        let keys = [ModKey::Ctrl, ModKey::Alt];
        assert_eq!(ModKey::mod_mask_from_slice(&keys), 0x05);

        let keys = [ModKey::Ctrl, ModKey::Shift, ModKey::Win];
        assert_eq!(ModKey::mod_mask_from_slice(&keys), 0x0B);

        let keys: [ModKey; 0] = [];
        assert_eq!(ModKey::mod_mask_from_slice(&keys), 0x00);
    }

    #[test]
    fn test_mod_mask_from_bool() {
        assert_eq!(ModKey::mod_mask_from_bool(true, false, true, false), 0x05);
        assert_eq!(ModKey::mod_mask_from_bool(true, true, false, true), 0x0B);
        assert_eq!(ModKey::mod_mask_from_bool(false, false, false, false), 0x00);
    }

    #[test]
    fn test_to_mod_bit() {
        assert_eq!(ModKey::Ctrl.to_mod_bit(), 0x01);
        assert_eq!(ModKey::Shift.to_mod_bit(), 0x02);
        assert_eq!(ModKey::Alt.to_mod_bit(), 0x04);
        assert_eq!(ModKey::Win.to_mod_bit(), 0x08);
    }

    #[test]
    fn test_combined_behavior() {
        // Test the full flow: from_keyname -> to_vk_code -> from_vk_code -> mod_mask_from_slice
        let keys = ["Ctrl", "Shift", "Alt"];
        let mod_keys: Vec<ModKey> = keys
            .iter()
            .map(|&key| ModKey::from_keyname(key).unwrap())
            .collect();
        let vk_codes: Vec<u16> = mod_keys.iter().map(|key| key.to_vk_code()).collect();
        let reconstructed_mod_keys: Vec<ModKey> = vk_codes
            .iter()
            .map(|&code| ModKey::from_vk_code(code).unwrap())
            .collect();
        assert_eq!(mod_keys, reconstructed_mod_keys);

        // Verify bitmask
        assert_eq!(ModKey::mod_mask_from_slice(&reconstructed_mod_keys), 0x07);
    }
}
