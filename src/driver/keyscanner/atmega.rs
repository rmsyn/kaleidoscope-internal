#[cfg(feature = "atreus")]
use crate::plugins::atreus::DeviceProps;

use super::{DummyScanner, KeyScannerProps};

/// Represents the debounce state
///
/// Each of these variables are storing the state for a row of keys
///
/// So for key 0, the counter is represented by db0[0] and db1[0]
/// and the state in debounced_state[0].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Debounce {
    /// Counter bit 0
    db0: u16,
    /// Counter bit 1
    db1: u16,
    /// Debounced state
    debounced_state: u16,
}

impl Debounce {
    /// Creates a default [Debounce]
    pub const fn default() -> Self {
        Self {
            db0: 0,
            db1: 0,
            debounced_state: 0,
        }
    }
}

/// Represents a matrix row state.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowState {
    /// Previous row state
    previous: u16,
    /// Current row state
    current: u16,
    /// Debounce state
    debouncer: Debounce,
}

impl RowState {
    /// Creates a default [RowState].
    pub const fn default() -> Self {
        Self {
            previous: 0,
            current: 0,
            debouncer: Debounce::default(),
        }
    }
}

/// Represents an Atmega-based KeyScanner.
pub struct Atmega {
    do_scan: bool,
    matrix_state: [RowState; DeviceProps::ROWS],
}

impl Atmega {
    /// Creates a new [Atmega] key scanner.
    pub const fn new() -> Self {
        Self {
            do_scan: true,
            matrix_state: [RowState::default(); DeviceProps::ROWS],
        }
    }

    /// Gets whether the scanner should scan the keys.
    pub fn do_scan(&self) -> bool {
        self.do_scan
    }

    /// Sets whether the scanner should scan the keys.
    pub fn set_do_scan(&mut self, do_scan: bool) {
        self.do_scan = do_scan;
    }

    /// Gets a reference to the [RowState] array.
    pub fn matrix_state(&self) -> &[RowState] {
        self.matrix_state.as_ref()
    }
}

impl DummyScanner for Atmega {}
