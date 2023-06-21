#[cfg(any(feature = "avr", feature = "atmega32u4"))]
mod avr;

#[cfg(any(feature = "avr", feature = "atmega32u4"))]
pub use avr::*;

pub mod key_indexes;

pub trait DeviceOps {
    type KeyScanner;

    fn key_scanner(&self) -> &Self::KeyScanner;
    fn key_scanner_mut(&mut self) -> &mut Self::KeyScanner;
}
