use super::*;

pub const LAYER_OP_OFFSET: u8 = 42;
pub const LAYER_SHIFT_OFFSET: u8 = LAYER_OP_OFFSET;
pub const LAYER_MOVE_OFFSET: u8 = LAYER_SHIFT_OFFSET + LAYER_OP_OFFSET;

pub const KEYMAP_PREVIOUS: u8 = 33;
pub const KEYMAP_NEXT: u8 = 34;

pub const fn shift_to_layer(n: u8) -> Key {
    Key::new(
        n + LAYER_SHIFT_OFFSET,
        KeyFlags(KEY_FLAGS | SYNTHETIC | SWITCH_TO_KEYMAP),
    )
}

pub const fn lock_layer(n: u8) -> Key {
    Key::new(n, KeyFlags(KEY_FLAGS | SYNTHETIC | SWITCH_TO_KEYMAP))
}

pub const fn move_to_layer(n: u8) -> Key {
    Key::new(
        n + LAYER_MOVE_OFFSET,
        KeyFlags(KEY_FLAGS | SYNTHETIC | SWITCH_TO_KEYMAP),
    )
}

#[macro_export]
macro_rules! MO {
    ($key:tt) => {
        $crate::key_defs::keymaps::shift_to_layer($key)
    };
}

#[macro_export]
macro_rules! TG {
    ($key:tt) => {
        $crate::key_defs::keymaps::lock_layer($key)
    };
}

#[macro_export]
macro_rules! ML {
    ($key:tt) => {
        $crate::key_defs::keymaps::move_to_layer($key)
    };
}
