use crate::error::WHKError;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::hash::Hash;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

macro_rules! vkeys_definition {
    ($($name:ident = $value:ident $(alias $alias:literal)?,)*) => {
        /// Represents a virtual key (VK) code.
        ///
        /// # See Also
        /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
        #[derive(Debug, Clone, Copy, FromPrimitive, IntoPrimitive)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(u16)]
        pub enum VKey {
            $(
                #[cfg_attr(feature = "serde", serde(alias = stringify!($value)$(,alias = $alias)?))]
                $name = $value.0,
            )*
            /// the scan code of the key has no mapping
            None = VK__none_.0,
            #[num_enum(catch_all)]
            UnknownOrReserved(u16),
        }

        impl VKey {
            fn vk_name_from_alias(alias: &str) -> Option<String> {
                $($(
                    if alias.eq_ignore_ascii_case($alias) {
                        return Some(stringify!($value).to_string());
                    }
                )?)*
                None
            }

            /// Creates a `VKey` from a string representation of the key.
            ///
            /// NOTE: Certain common aliases for keys are accepted in addition to the Microsoft Virtual-Key Codes names
            ///
            /// WIN maps to `VKey::LWin`
            /// CTRL maps to `VKey::Control`
            /// ALT maps to `VKey::Menu`
            ///
            /// # See Also
            /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
            ///
            pub fn from_keyname(key_name: &str) -> Result<VKey, WHKError> {
                if let Some(key) = VKey::from_maybe_hex_string(key_name) {
                    return Ok(key);
                }

                let vk_name = match Self::vk_name_from_alias(key_name) {
                    Some(name) => name,
                    None => {
                        let mut vk_name = key_name.to_ascii_uppercase();
                        if !vk_name.starts_with("VK_") {
                            vk_name = format!("VK_{}", vk_name);
                        }
                        vk_name
                    },
                };

                let key = match vk_name.as_str() {
                    $(
                      stringify!($value) => VKey::$name,
                    )*
                    _ => return Err(WHKError::InvalidKey(key_name.to_string())),
                };

                Ok(key)
            }
        }
    };
}

vkeys_definition! {
    Back = VK_BACK,
    Tab = VK_TAB,
    Clear = VK_CLEAR,
    Return = VK_RETURN,
    Shift = VK_SHIFT,
    Control = VK_CONTROL alias "Ctrl",
    Menu = VK_MENU alias "Alt",
    Pause = VK_PAUSE,
    Capital = VK_CAPITAL,
    Escape = VK_ESCAPE,
    Space = VK_SPACE,
    Prior = VK_PRIOR,
    Next = VK_NEXT,
    End = VK_END,
    Home = VK_HOME,
    Left = VK_LEFT,
    Up = VK_UP,
    Right = VK_RIGHT,
    Down = VK_DOWN,
    Select = VK_SELECT,
    Print = VK_PRINT,
    Execute = VK_EXECUTE,
    Snapshot = VK_SNAPSHOT,
    Insert = VK_INSERT,
    Delete = VK_DELETE,
    Help = VK_HELP,
    LWin = VK_LWIN alias "Win",
    RWin = VK_RWIN,
    Apps = VK_APPS,
    Sleep = VK_SLEEP,
    Numpad0 = VK_NUMPAD0,
    Numpad1 = VK_NUMPAD1,
    Numpad2 = VK_NUMPAD2,
    Numpad3 = VK_NUMPAD3,
    Numpad4 = VK_NUMPAD4,
    Numpad5 = VK_NUMPAD5,
    Numpad6 = VK_NUMPAD6,
    Numpad7 = VK_NUMPAD7,
    Numpad8 = VK_NUMPAD8,
    Numpad9 = VK_NUMPAD9,
    Multiply = VK_MULTIPLY,
    Add = VK_ADD,
    Separator = VK_SEPARATOR,
    Subtract = VK_SUBTRACT,
    Decimal = VK_DECIMAL,
    Divide = VK_DIVIDE,
    F1 = VK_F1,
    F2 = VK_F2,
    F3 = VK_F3,
    F4 = VK_F4,
    F5 = VK_F5,
    F6 = VK_F6,
    F7 = VK_F7,
    F8 = VK_F8,
    F9 = VK_F9,
    F10 = VK_F10,
    F11 = VK_F11,
    F12 = VK_F12,
    F13 = VK_F13,
    F14 = VK_F14,
    F15 = VK_F15,
    F16 = VK_F16,
    F17 = VK_F17,
    F18 = VK_F18,
    F19 = VK_F19,
    F20 = VK_F20,
    F21 = VK_F21,
    F22 = VK_F22,
    F23 = VK_F23,
    F24 = VK_F24,
    Numlock = VK_NUMLOCK,
    Scroll = VK_SCROLL,
    LShift = VK_LSHIFT,
    RShift = VK_RSHIFT,
    LControl = VK_LCONTROL alias "LCtrl",
    RControl = VK_RCONTROL alias "RCtrl",
    LMenu = VK_LMENU alias "LAlt",
    RMenu = VK_RMENU alias "RAlt",
    BrowserBack = VK_BROWSER_BACK,
    BrowserForward = VK_BROWSER_FORWARD,
    BrowserRefresh = VK_BROWSER_REFRESH,
    BrowserStop = VK_BROWSER_STOP,
    BrowserSearch = VK_BROWSER_SEARCH,
    BrowserFavorites = VK_BROWSER_FAVORITES,
    BrowserHome = VK_BROWSER_HOME,
    VolumeMute = VK_VOLUME_MUTE,
    VolumeDown = VK_VOLUME_DOWN,
    VolumeUp = VK_VOLUME_UP,
    MediaNextTrack = VK_MEDIA_NEXT_TRACK,
    MediaPrevTrack = VK_MEDIA_PREV_TRACK,
    MediaStop = VK_MEDIA_STOP,
    MediaPlayPause = VK_MEDIA_PLAY_PAUSE,
    LaunchMail = VK_LAUNCH_MAIL,
    LaunchMediaSelect = VK_LAUNCH_MEDIA_SELECT,
    LaunchApp1 = VK_LAUNCH_APP1,
    LaunchApp2 = VK_LAUNCH_APP2,
    Oem1 = VK_OEM_1,
    OemPlus = VK_OEM_PLUS,
    OemComma = VK_OEM_COMMA,
    OemMinus = VK_OEM_MINUS,
    OemPeriod = VK_OEM_PERIOD,
    Oem2 = VK_OEM_2,
    Oem3 = VK_OEM_3,
    Oem4 = VK_OEM_4,
    Oem5 = VK_OEM_5,
    Oem6 = VK_OEM_6,
    Oem7 = VK_OEM_7,
    Oem8 = VK_OEM_8,
    Oem102 = VK_OEM_102,
    Attn = VK_ATTN,
    Crsel = VK_CRSEL,
    Exsel = VK_EXSEL,
    Play = VK_PLAY,
    Zoom = VK_ZOOM,
    Pa1 = VK_PA1,
    OemClear = VK_OEM_CLEAR,
    Vk0 = VK_0 alias "0",
    Vk1 = VK_1 alias "1",
    Vk2 = VK_2 alias "2",
    Vk3 = VK_3 alias "3",
    Vk4 = VK_4 alias "4",
    Vk5 = VK_5 alias "5",
    Vk6 = VK_6 alias "6",
    Vk7 = VK_7 alias "7",
    Vk8 = VK_8 alias "8",
    Vk9 = VK_9 alias "9",
    A = VK_A,
    B = VK_B,
    C = VK_C,
    D = VK_D,
    E = VK_E,
    F = VK_F,
    G = VK_G,
    H = VK_H,
    I = VK_I,
    J = VK_J,
    K = VK_K,
    L = VK_L,
    M = VK_M,
    N = VK_N,
    O = VK_O,
    P = VK_P,
    Q = VK_Q,
    R = VK_R,
    S = VK_S,
    T = VK_T,
    U = VK_U,
    V = VK_V,
    W = VK_W,
    X = VK_X,
    Y = VK_Y,
    Z = VK_Z,
}

impl VKey {
    /// Converts a `VKey` to its corresponding Windows Virtual-Key (VK) code.
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    ///
    pub fn to_vk_code(&self) -> u16 {
        u16::from(*self)
    }

    /// Returns a `VKey` based a Windows Virtual-Key (VK) code.
    ///
    /// # See Also
    /// - [Microsoft Virtual-Key Codes](https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
    ///
    pub fn from_vk_code(vk_code: u16) -> VKey {
        VKey::from(vk_code)
    }

    fn from_maybe_hex_string(name: &str) -> Option<VKey> {
        // 1 byte hex code => Use the raw keycode value
        if name.len() >= 3 && name.len() <= 6 && name.starts_with("0x") || name.starts_with("0X") {
            return if let Ok(val) = u16::from_str_radix(&name[2..], 16) {
                Some(Self::from_vk_code(val))
            } else {
                None
            };
        }
        None
    }
}

impl std::str::FromStr for VKey {
    type Err = WHKError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VKey::from_keyname(s)
    }
}

impl Eq for VKey {}
impl PartialEq<VKey> for VKey {
    fn eq(&self, other: &VKey) -> bool {
        self.to_vk_code() == other.to_vk_code()
    }
}

impl Hash for VKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_vk_code().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_vk_code() {
        assert_eq!(VKey::Back.to_vk_code(), VK_BACK.0);
        assert_eq!(VKey::Return.to_vk_code(), VK_RETURN.0);
        assert_eq!(VKey::Space.to_vk_code(), VK_SPACE.0);
        assert_eq!(VKey::F12.to_vk_code(), VK_F12.0);
        assert_eq!(VKey::UnknownOrReserved(1234).to_vk_code(), 1234); // Unknown key
    }

    #[test]
    fn test_from_keyname() {
        assert_eq!(VKey::from_keyname("BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_keyname("VK_BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_keyname("RETURN").unwrap(), VKey::Return);
        assert_eq!(VKey::from_keyname("0x29").unwrap(), VKey::Select);
        assert_eq!(VKey::from_keyname("0x29").unwrap(), VKey::UnknownOrReserved(0x29));
        assert_eq!(
            VKey::from_keyname("0xff1").unwrap(),
            VKey::UnknownOrReserved(0xff1)
        );
        assert!(VKey::from_keyname("INVALID_KEY").is_err());
    }

    // Eythan note: I think we shouldn't allow convertion from VKey to string, directly
    // for this `serde` feature was added and is a better approach if clients want to serialize
    /* #[test]
    fn test_display() {
        assert_eq!(format!("{}", VKey::Back), "VK_BACK");
        assert_eq!(format!("{}", VKey::Return), "VK_RETURN");
        assert_eq!(format!("{}", VKey::CustomKeyCode(1234)), "Custom(1234)");
    } */

    #[test]
    fn test_from_str() {
        use std::str::FromStr;
        assert_eq!(VKey::from_str("BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_str("VK_BACK").unwrap(), VKey::Back);
        assert_eq!(VKey::from_str("INVALID_KEY").is_err(), true);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(VKey::Back, VKey::Back); // Identical keys
        assert_eq!(
            VKey::UnknownOrReserved(1234),
            VKey::UnknownOrReserved(1234)
        ); // Same key
        assert_ne!(
            VKey::UnknownOrReserved(1234),
            VKey::UnknownOrReserved(5678)
        ); // Different keys
    }

    #[test]
    fn test_custom_keycode_range() {
        assert_eq!(VKey::UnknownOrReserved(0).to_vk_code(), 0);
        assert_eq!(VKey::UnknownOrReserved(65535).to_vk_code(), 65535); // Maximum value for u16
    }
}
