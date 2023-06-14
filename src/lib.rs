//! Common types and functionality for Kaleidoscope libraries.
//!
//! Separated out to make unit and integration testing easier.
//!
//! Structures and functionality do not depend on hardware-specific features, allowing for
//! platform-independent testing.

#![no_std]

#[macro_use(bitfield)]
extern crate bitfield;

// Re-exports
pub use usbd_hid;

/// HID table constants
pub mod hid_tables;

/// Key definitions
pub mod key_defs;

/// Matrix address
pub mod matrix_addr;
