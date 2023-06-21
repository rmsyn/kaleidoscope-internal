/// Keyboardio Atreus hardware support
#[cfg(any(feature = "atreus", feature = "atmega32u4"))]
pub mod atreus;
pub mod macros;
pub mod ranges;
