# Kaleidoscope internal library

Kaleidoscope is firmware built for embedded devices. To help with testing, hardware-independent data structures and functionality are moved into this library.

This library should be testable on any target platform, which greatly eases testing by not requiring custom test runners, emulated test environments, and/or real target hardware.

Hardware-specific tests will remain inside the main [kaleidoscope-rs](https://github.com/rmsyn/kaleidoscope-rs) library.
