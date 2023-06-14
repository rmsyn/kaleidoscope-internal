use core::ptr::{read_volatile, write_volatile};

pub mod pins_and_ports;

/// ATmega32u4 CPU frequency: 16MHz
pub const F_CPU: u32 = 16_000_000;

/// SFR offset from avr-libc
pub const SFR_OFFSET: u8 = 0x20;

pub const EPEN: u8 = 0;

pub const EPDIR: u8 = 0;
pub const EPTYPE0: u8 = 6;
pub const EPTYPE1: u8 = 7;

#[cfg(feature = "atmega32u4")]
pub const RAMSTART: usize = 0x100;
#[cfg(feature = "atmega32u4")]
pub const RAMSIZE: usize = 0xA00;
/// Last On-Chip SRAM Location
#[cfg(feature = "atmega32u4")]
pub const RAMEND: usize = RAMSTART + RAMSIZE - 1;
#[cfg(feature = "atmega32u4")]
pub const FLASHEND: u16 = 0x7fff;

/// Get the SFR address
pub const fn sfr_addr(addr: u8) -> u8 {
    addr + SFR_OFFSET
}

/// Read memory from the provided address
pub unsafe fn sfr_read8(addr: u8) -> u8 {
    read_volatile(sfr_addr(addr) as *const u8)
}

/// Write a value to memory at the provided address
pub unsafe fn sfr_write8(addr: u8, value: u8) {
    write_volatile(sfr_addr(addr) as *mut u8, value);
}

/// Get the bit value
pub const fn bv(bit: u8) -> u8 {
    debug_assert!(bit < 8);

    1 << bit
}
