#[cfg(any(feature = "avr", feature = "atmega32u4"))]
pub mod avr;

pub trait Base {
    fn setup() {}
    fn reboot_bootloader() -> ! {
        loop {}
    }
}
