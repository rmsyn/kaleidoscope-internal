/// Magic bits to reboot into the bootloader, and stay there.
pub const BOOT_KEY: u16 = 0x7777;
pub const BOOT_KEY_PTR: u16 = 0x0800;

/// Magic value of the AVR bootloader boot key.
pub const fn boot_key() -> u16 {
    BOOT_KEY
}

/// Pointer value to the boot key in AVR bootloader memory.
pub const fn boot_key_ptr() -> u16 {
    BOOT_KEY_PTR
}

pub struct Caterina;
