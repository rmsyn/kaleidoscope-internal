#[cfg(feature = "atmega32u4")]
mod atmega;
#[cfg(feature = "atmega32u4")]
pub use atmega::*;

/// Trait for common Keyscanner property constants
pub trait KeyScannerProps {
    const ROWS: usize;
    const COLS: usize;

    const KEYSCAN_INTERVAL: u16;
}

/// Dummy trait to scan the key matrix.
///
/// Users of this crate should use a similar trait that provides the `scan_matrix` function,
/// with impls that actually do the matrix scan.
pub trait DummyScanner {
    fn scan_matrix(&mut self) {}
}
