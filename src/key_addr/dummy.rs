use crate::matrix_addr::MatrixAddr;

pub const ROWS: u8 = 0xffu8;
pub const COLS: u8 = 0xffu8;

pub type KeyAddr = MatrixAddr<ROWS, COLS>;
