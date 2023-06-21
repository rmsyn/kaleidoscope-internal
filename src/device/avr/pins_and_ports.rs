use super::{bv, sfr_read8, sfr_write8};

// FIXME: this is a more-or-less direct port from the C++ library.
// Implement the below GPIO pins/ports with concepts from the [embedded-book](https://doc.rust-lang.org/stable/embedded-book/).

/// This value is the same for all AVR chips
pub const PORT_SHIFTER: u8 = 4;

/// Pin address mask, ensures 4-bit pin addresses
pub const PIN_ADDRESS_MASK: u8 = 0xF;

/// Base address for ports and pins
pub const ADDRESS_BASE: u8 = 0x00;

/// Pin A address
pub const PINA_ADDRESS: u8 = 0x0;
/// Pin B address
pub const PINB_ADDRESS: u8 = 0x3;
/// Pin C address
pub const PINC_ADDRESS: u8 = 0x6;
/// Pin D address
pub const PIND_ADDRESS: u8 = 0x9;
/// Pin E address
pub const PINE_ADDRESS: u8 = 0xC;
/// Pin F address
pub const PINF_ADDRESS: u8 = 0xF;

/// AVR pin definitions
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pin {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
}

impl From<u8> for Pin {
    fn from(b: u8) -> Self {
        match b % 8 {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            _ => unreachable!("invalid Pin"),
        }
    }
}

/// AVR port definitions
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Port {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}

/// AVR address offsets
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Offset {
    Pin = 0,
    Ddr = 1,
    Port = 2,
}

/// Define pin address relative to port offsets
pub const fn pindef(port: Port, pin: Pin) -> u8 {
    let addr = match port {
        Port::A => PINA_ADDRESS as u8,
        Port::B => PINB_ADDRESS as u8,
        Port::C => PINC_ADDRESS as u8,
        Port::D => PIND_ADDRESS as u8,
        Port::E => PINE_ADDRESS as u8,
        Port::F => PINF_ADDRESS as u8,
    };
    (addr << PORT_SHIFTER) | pin as u8
}

use paste::paste;
macro_rules! pin_constants {
    ($pin:ident) => {
        paste! {
            pub const [<PIN_ $pin 0>]: u8 = pindef(Port::$pin, Pin::Zero);
            pub const [<PIN_ $pin 1>]: u8 = pindef(Port::$pin, Pin::One);
            pub const [<PIN_ $pin 2>]: u8 = pindef(Port::$pin, Pin::Two);
            pub const [<PIN_ $pin 3>]: u8 = pindef(Port::$pin, Pin::Three);
            pub const [<PIN_ $pin 4>]: u8 = pindef(Port::$pin, Pin::Four);
            pub const [<PIN_ $pin 5>]: u8 = pindef(Port::$pin, Pin::Five);
            pub const [<PIN_ $pin 6>]: u8 = pindef(Port::$pin, Pin::Six);
            pub const [<PIN_ $pin 7>]: u8 = pindef(Port::$pin, Pin::Seven);
        }
    };
}

pin_constants!(A);
pin_constants!(B);
pin_constants!(C);
pin_constants!(D);
pin_constants!(E);
pin_constants!(F);

/// Get the pin address relative to the address base, port shift, and offset
pub const fn pin_address(pin: Pin, offset: Offset) -> u8 {
    ADDRESS_BASE + (pin as u8 >> PORT_SHIFTER) + offset as u8
}

/// Read the value at the pin address
pub fn pin_address_value(pin: Pin, offset: Offset) -> u8 {
    unsafe { sfr_read8(pin_address(pin, offset)) }
}

/// Get the pin register value for the pin offset
pub fn pin_reg_for_pin(pin: Pin) -> u8 {
    pin_address_value(pin, Offset::Pin)
}

/// Set the pin register value for the pin offset
pub fn set_pin_reg_for_pin(pin: Pin, value: u8) {
    unsafe {
        sfr_write8(pin_address(pin, Offset::Pin), value);
    }
}

/// Get the pin address for the DDR offset
pub fn ddr_reg_for_pin(pin: Pin) -> u8 {
    pin_address(pin, Offset::Ddr)
}

/// Set the pin address for the DDR offset
pub fn set_ddr_reg_for_pin(pin: Pin, value: u8) {
    unsafe {
        sfr_write8(pin_address_value(pin, Offset::Ddr), value);
    }
}

/// Get the pin value for the port register
pub fn port_reg_for_pin(pin: Pin) -> u8 {
    pin_address_value(pin, Offset::Port)
}

/// Set the pin value for the port register
pub fn set_port_reg_for_pin(pin: Pin, value: u8) {
    unsafe {
        sfr_write8(pin_address(pin, Offset::Port), value);
    }
}

/// Get the pin number
pub const fn pin_num_for_pin(pin: Pin) -> u8 {
    pin as u8 & PIN_ADDRESS_MASK
}

/// Get the pin mask for the pin
pub const fn pin_mask_for_pin(pin: Pin) -> u8 {
    bv(pin_num_for_pin(pin))
}

/// DDR input
pub fn ddr_input(pin: Pin) {
    set_ddr_reg_for_pin(pin, ddr_reg_for_pin(pin) & !pin_mask_for_pin(pin));
}

/// DDR output
pub fn ddr_output(pin: Pin) {
    set_ddr_reg_for_pin(pin, ddr_reg_for_pin(pin) | pin_mask_for_pin(pin));
}

/// Enable pullup for an input pin
pub fn enable_pullup(pin: Pin) {
    set_port_reg_for_pin(pin, port_reg_for_pin(pin) | pin_mask_for_pin(pin));
}

/// Disable pullup for an input pin
pub fn disable_pullup(pin: Pin) {
    set_port_reg_for_pin(pin, port_reg_for_pin(pin) & !pin_mask_for_pin(pin));
}

/// Drive the pin high for an output pin
pub fn drive_output_high(pin: Pin) {
    set_port_reg_for_pin(pin, port_reg_for_pin(pin) | pin_mask_for_pin(pin))
}

/// Drive the pin low for an output pin
pub fn drive_output_low(pin: Pin) {
    set_port_reg_for_pin(pin, port_reg_for_pin(pin) & !pin_mask_for_pin(pin))
}

/// Toggle the pin state for an output pin
pub fn output_toggle(pin: Pin) {
    set_port_reg_for_pin(pin, port_reg_for_pin(pin) ^ pin_mask_for_pin(pin))
}

/// Read the value of a pin
pub fn read_pin(pin: Pin) -> u8 {
    pin_reg_for_pin(pin) & pin_mask_for_pin(pin) & 0b1
}
