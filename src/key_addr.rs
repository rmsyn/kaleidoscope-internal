#[cfg(feature = "atreus")]
mod atreus;
#[cfg(any(not(feature = "atreus"), not(feature = "atmega32u4"), not(feature = "avr")))]
mod dummy;

#[cfg(feature = "atreus")]
pub use atreus::*;
#[cfg(any(not(feature = "atreus"), not(feature = "atmega32u4"), not(feature = "avr")))]
pub use dummy::*;
