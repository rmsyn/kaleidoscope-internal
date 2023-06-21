use crate::driver::keyscanner::KeyScannerProps;
use crate::matrix_addr::MatrixAddr;
use crate::plugins::atreus::DeviceProps;

pub const ROWS: u8 = DeviceProps::ROWS as u8;
pub const COLS: u8 = DeviceProps::COLS as u8;

pub type KeyAddr = MatrixAddr<ROWS, COLS>;
