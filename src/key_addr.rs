#[cfg(feature = "atreus")]
mod atreus;
#[cfg(not(any(feature = "atreus", feature = "avr", feature = "atmega32u4")))]
mod dummy;

#[cfg(feature = "atreus")]
pub use atreus::*;
#[cfg(not(any(feature = "atreus", feature = "avr", feature = "atmega32u4")))]
pub use dummy::*;
