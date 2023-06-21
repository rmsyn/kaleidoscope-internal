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

/// Device definitions
pub mod device;

/// Driver-related types and functionality
pub mod driver;

/// HID table constants
pub mod hid_tables;

/// Key address definitions
pub mod key_addr;

/// Key definitions
pub mod key_defs;

/// Matrix address
pub mod matrix_addr;

/// Board-specific plugins
pub mod plugins;
