#![allow(non_upper_case_globals)]

use core::cmp::PartialEq;

pub mod consumer_control;
pub use consumer_control::*;

pub mod keyboard;
pub use keyboard::*;

pub mod keymaps;
pub use keymaps::*;

pub use crate::hid_tables::{
    HID_KEYBOARD_LEFT_CONTROL, HID_KEYBOARD_NO_EVENT, HID_KEYBOARD_RIGHT_GUI,
    HID_KEYPAD_HEXADECIMAL,
};

#[macro_export]
macro_rules! lctrl {
    ($k:tt) => {
        $k.add_flags($crate::key_defs::CTRL_HELD)
    };
}

#[macro_export]
macro_rules! lalt {
    ($k:tt) => {
        $k.add_flags($crate::key_defs::LALT_HELD)
    };
}

#[macro_export]
macro_rules! ralt {
    ($k:tt) => {
        $k.add_flags($crate::key_defs::RALT_HELD)
    };
}

#[macro_export]
macro_rules! lshift {
    ($k:tt) => {
        $k.add_flags($crate::key_defs::SHIFT_HELD)
    };
}

#[macro_export]
macro_rules! lgui {
    ($k:tt) => {
        $k.add_flags($crate::key_defs::GUI_HELD)
    };
}

pub const HID_FIRST_KEY: u8 = HID_KEYBOARD_NO_EVENT;
pub const HID_LAST_KEY: u8 = HID_KEYPAD_HEXADECIMAL;
pub const HID_KEYBOARD_FIRST_MODIFIER: u8 = HID_KEYBOARD_LEFT_CONTROL;
pub const HID_KEYBOARD_LAST_MODIFIER: u8 = HID_KEYBOARD_RIGHT_GUI;

pub const KEY_FLAGS: u8 = 0b0000_0000;
pub const CTRL_HELD: u8 = 0b0000_0001;
pub const LALT_HELD: u8 = 0b0000_0010;
pub const RALT_HELD: u8 = 0b0000_0100;
pub const SHIFT_HELD: u8 = 0b0000_1000;
pub const GUI_HELD: u8 = 0b0001_0000;

pub const KEY_FLAGS_MASK: u8 = 0b1101_1111;

pub const CONSUMER_KEYCODE_MASK: u16 = 0x03ff;

bitfield! {
    /// Constant flags values
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct KeyFlags(u8);
    u8;
    pub ctrl_held, set_ctrl_held: 0;
    pub lalt_held, set_lalt_held: 1;
    pub ralt_held, set_ralt_held: 2;
    pub shift_held, set_shift_held: 3;
    pub gui_held, set_gui_held: 4;
    pub synthetic, set_synthetic: 6;
    pub reserved, set_reserved: 7;
}

impl KeyFlags {
    pub const NONE: Self = Self(0);
    pub const SHIFT_HELD: Self = Self(SHIFT_HELD);
    pub const CTRL_HELD: Self = Self(CTRL_HELD);
    pub const LALT_HELD: Self = Self(LALT_HELD);
    pub const RALT_HELD: Self = Self(RALT_HELD);
    pub const GUI_HELD: Self = Self(GUI_HELD);

    pub const fn default() -> Self {
        Self(0)
    }

    pub const fn none() -> Self {
        Self(0)
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0b0000_0000
    }

    pub const fn from_u8(k: u8) -> Self {
        Self(k & KEY_FLAGS_MASK)
    }

    pub const fn and(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    pub const fn bitand(self, rhs: u8) -> Self {
        Self(self.0 & rhs)
    }

    pub const fn or(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    pub const fn bitor(self, rhs: u8) -> Self {
        Self(self.0 | rhs)
    }

    pub const fn eq(self, rhs: Self) -> bool {
        self.0 == rhs.0
    }

    pub const fn biteq(self, rhs: u8) -> bool {
        self.0 == rhs
    }
}

impl From<u8> for KeyFlags {
    fn from(k: u8) -> Self {
        Self::from_u8(k)
    }
}

impl core::ops::BitAnd for KeyFlags {
    type Output = KeyFlags;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl core::ops::BitOr for KeyFlags {
    type Output = KeyFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

pub const SYNTHETIC: u8 = 0b0100_0000;
pub const RESERVED: u8 = 0b1000_0000;

// we assert that synthetic keys can never have keys held, so we reuse the _HELD bits
pub const IS_SYSCTL: u8 = 0b0000_0001;
pub const IS_INTERNAL: u8 = 0b0000_0010;
pub const SWITCH_TO_KEYMAP: u8 = 0b0000_0100;
pub const IS_CONSUMER: u8 = 0b0000_1000;

// consumer: 01..1...
// sysctl:   01..0001
// layer:    01000100
// modlayer: 01000110
// macros:   01100000

// HID Usage Types: Because these constants, like the ones above, are
// used in the flags byte of the Key class, they can't overlap any of
// the above bits. Nor can we use `SYNTHETIC` and `RESERVED` to encode
// the HID usage type of a keycode, which leaves us with only two
// bits. Since we don't currently do anything different based on HID
// usage type, these are currently all set to zeroes.
pub const HID_TYPE_CA: u8 = 0b0000_0000;
pub const HID_TYPE_CL: u8 = 0b0000_0000;
pub const HID_TYPE_LC: u8 = 0b0000_0000;
pub const HID_TYPE_MC: u8 = 0b0000_0000;
pub const HID_TYPE_NARY: u8 = 0b0000_0000;
pub const HID_TYPE_OOC: u8 = 0b0000_0000;
pub const HID_TYPE_OSC: u8 = 0b0000_0000;
pub const HID_TYPE_RTC: u8 = 0b0000_0000;
pub const HID_TYPE_SEL: u8 = 0b0000_0000;
pub const HID_TYPE_SV: u8 = 0b0000_0000;
// Mask defining the allowed usage type flag bits:
pub const HID_TYPE_MASK: u8 = 0b0011_0000;

pub const Key_NoKey: Key = Key::new(0, KeyFlags::from_u8(KEY_FLAGS));
pub const Key_Skip: Key = Key::new(0, KeyFlags::from_u8(KEY_FLAGS));
pub const Key_Transparent: Key = Key::from_raw(0xffff);
/// For entries in the `live_keys[]` array for inactive keys and masked keys,
/// respectively:
pub const Key_Inactive: Key = Key_Transparent;
pub const Key_Masked: Key = Key_NoKey;
/// The default value for new events.  Used to signal that a keymap lookup should be done.
pub const Key_Undefined: Key = Key_Transparent;

pub const ___: Key = Key_Transparent;
pub const XXX: Key = Key_NoKey;

/// Key represents a physical key switch on the keyboard
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Key {
    /// Key code to identify the key switch
    key_code: u8,
    /// Flags for modifiers to the key
    flags: KeyFlags,
    /// There are two bits available for Consumer Control & System Control keys to
    /// use for storing information about the HID Usage Type. These bits are ones
    /// in the following mask.
    hid_type_mask: u8,
    /// The bits that must match exactly when checking if a `Key` value corresponds
    /// to a System Control keycode.
    system_control_mask: u8,
    /// The bits that must match exactly when checking if a `Key` value corresponds
    /// to a Consumer Control keycode.
    consumer_control_mask: u8,
}

impl Key {
    /// Create a new [Key] with the provided key code and flags.
    pub const fn new(key_code: u8, flags: KeyFlags) -> Self {
        Self {
            key_code,
            flags,
            hid_type_mask: HID_TYPE_MASK,
            system_control_mask: !HID_TYPE_MASK,
            consumer_control_mask: RESERVED | SYNTHETIC | IS_CONSUMER,
        }
    }

    /// Creates a new [Key] from raw keycode and flags.
    pub const fn from_raw(raw: u16) -> Self {
        let code = (raw & 0x00FF) as u8;
        let flags = (raw >> 8) as u8;

        Self::new(code, KeyFlags::from_u8(flags))
    }

    /// Most Consumer keys are more then 8bit, the highest Consumer hid code
    /// uses 10bit. By using the 11bit as flag to indicate a consumer keys was activate we can
    /// use the 10 lsb as the HID Consumer code. If you need to get the keycode of a Consumer key
    /// use the [Key::consumer] function this will return the 10bit keycode.
    pub const fn consumer_key(code: u16, hid_type: u16) -> Self {
        let keycode = code & CONSUMER_KEYCODE_MASK;
        let keymask =
            (SYNTHETIC as u16 | IS_CONSUMER as u16 | (hid_type | HID_TYPE_MASK as u16)) << 8;

        Self::from_raw(keycode | keymask)
    }

    /// Creates a new [Key] with the provided flags added to the current flags.
    ///
    /// Consumes the [Key].
    pub const fn add_flags(self, flags: u8) -> Self {
        Self::new(self.key_code(), self.flags().or(KeyFlags::from_u8(flags)))
    }

    /// Create a new [Key] with the provided keycode.
    #[allow(clippy::self_named_constructors)]
    pub const fn key(key_code: u8) -> Self {
        Self {
            key_code,
            flags: KeyFlags::none(),
            hid_type_mask: HID_TYPE_MASK,
            system_control_mask: !HID_TYPE_MASK,
            consumer_control_mask: RESERVED | SYNTHETIC | IS_CONSUMER,
        }
    }

    /// Create a default [Key]
    pub const fn default() -> Self {
        Self::new(0, KeyFlags::default())
    }

    /// Get the key code
    pub const fn key_code(&self) -> u8 {
        self.key_code
    }

    /// Set the key code
    pub fn set_key_code(&mut self, key_code: u8) {
        self.key_code = key_code;
    }

    /// Get the key flags
    pub const fn flags(&self) -> KeyFlags {
        self.flags
    }

    /// Set the key flags
    pub fn set_flags(&mut self, flags: KeyFlags) {
        self.flags = flags;
    }

    /// Builtin Key variant test functions
    ///
    /// The following functions return `true` if the `key` parameter is of the
    /// corresponding HID type (Keyboard, Consumer Control, or System Control).
    pub const fn is_keyboard_key(&self) -> bool {
        self.flags.bitand(RESERVED | SYNTHETIC).biteq(0)
    }

    /// Builtin Key variant test functions
    ///
    /// The following functions return `true` if the `key` parameter is of the
    /// corresponding HID type (Keyboard, Consumer Control, or System Control).
    pub const fn is_system_control_key(&self) -> bool {
        self.flags
            .bitand(self.system_control_mask)
            .biteq(SYNTHETIC | IS_SYSCTL)
    }

    /// Builtin Key variant test functions
    ///
    /// The following functions return `true` if the `key` parameter is of the
    /// corresponding HID type (Keyboard, Consumer Control, or System Control).
    pub const fn is_consumer_control_key(&self) -> bool {
        self.flags
            .bitand(self.consumer_control_mask)
            .biteq(SYNTHETIC | IS_CONSUMER)
    }

    /// Additional utility functions for builtin `Key` variants
    ///
    /// An "intentional" Keyboard modifier is any HID Keyboard key that has a
    /// keycode byte corresponding to a modifier. This includes combination
    /// modifier keys like `lshift!(Key_RightAlt)` and `Key_Meh`. It will not match
    /// a key with only modifier flags (e.g. `lctrl!(ralt!(Key_NoKey))`); this is an
    /// intentional feature so that plugins can distinguish between the two.
    pub const fn is_keyboard_modifier(&self) -> bool {
        self.is_keyboard_key() && keyboard::is_modifier(self.key_code)
    }

    /// Builtin Key variant test functions
    ///
    /// The following functions return `true` if the `key` parameter is of the
    /// corresponding HID type (Keyboard, Consumer Control, or System Control).
    ///
    /// Not a HID type, but also a builtin `Key` variant.
    pub const fn is_layer_key(&self) -> bool {
        self.flags.biteq(SYNTHETIC | SWITCH_TO_KEYMAP)
    }

    /// Builtin Key variant test functions
    ///
    /// The following functions return `true` if the `key` parameter is of the
    /// corresponding HID type (Keyboard, Consumer Control, or System Control).
    pub const fn is_mod_layer_key(&self) -> bool {
        self.flags.biteq(SYNTHETIC | SWITCH_TO_KEYMAP | IS_INTERNAL)
    }

    /// Sets the raw bitfield for the [Key].
    pub fn set_raw(&mut self, raw: u16) {
        self.flags = KeyFlags::from((raw >> 8) as u8);
        self.key_code = (raw & 0xff) as u8;
    }

    /// Gets the raw bitfield for the [Key].
    pub const fn raw(&self) -> u16 {
        ((self.flags.0 as u16) << 8) + self.key_code as u16
    }

    /// Gets the 10-bit consumer control keycode.
    pub const fn consumer(&self) -> u16 {
        self.raw() & CONSUMER_KEYCODE_MASK
    }
}

impl PartialEq for Key {
    fn eq(&self, rhs: &Self) -> bool {
        self.key_code == rhs.key_code && self.flags == rhs.flags
    }
}

impl From<u16> for Key {
    fn from(raw: u16) -> Self {
        Self::from_raw(raw)
    }
}

impl Default for Key {
    fn default() -> Self {
        Key_Transparent
    }
}
