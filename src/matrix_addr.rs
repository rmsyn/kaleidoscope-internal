use core::ops::AddAssign;

/// Key matrix address space, with `ROWS` and `COLS` invariants for the number of rows and columns.
#[derive(Clone, Copy, Debug)]
pub struct MatrixAddr<const ROWS: u8, const COLS: u8> {
    offset: u8,
}

impl<const ROWS: u8, const COLS: u8> MatrixAddr<ROWS, COLS> {
    pub const INVALID_STATE: u8 = 255;
    pub const UPPER_LIMIT: u8 = ROWS * COLS;

    const fn invariant() {
        assert!(
            (ROWS * COLS) < 255,
            "The number of rows and columns provided to instantiate \n
                                    MatrixAddr<rows, cols> exceeds the supported total number \n
                                    of 255 keys"
        );
    }

    /// Creates a new [MatrixAddr] with the given offset.
    pub const fn new(offset: u8) -> Self {
        Self::invariant();

        Self { offset }
    }

    /// Creates a new [MatrixAddr] with the given row and column offsets
    pub const fn create(row: u8, col: u8) -> Self {
        Self::invariant();

        Self {
            offset: Self::to_int(row, col),
        }
    }

    /// Creates a default [MatrixAddr].
    pub const fn default() -> Self {
        Self::invariant();

        Self { offset: ROWS * COLS }
    }

    /// Creates a [MatrixAddr] from another [MatrixAddr].
    pub const fn from_addr<const R: u8, const C: u8>(rhs: MatrixAddr<R, C>) -> Self {
        let offset = if rhs.offset <= ROWS.saturating_mul(COLS).saturating_add(COLS) {
            rhs.offset
        } else {
            ROWS * COLS - 1
        };

        Self { offset }
    }

    pub fn iter() -> MatrixAddrIter<ROWS, COLS> {
        Self::new(0).into_iter()
    }

    /// Create an offset into a MatrixAddr
    pub const fn to_int(row: u8, col: u8) -> u8 {
        Self::invariant();

        row.overflowing_mul(COLS).0.overflowing_add(col).0
    }

    pub const fn index(&self) -> usize {
        Self::to_int(self.row(), self.col()) as usize
    }

    /// Get the offset
    pub fn offset(&self) -> u8 {
        self.offset
    }

    /// Get the row
    pub const fn row(&self) -> u8 {
        self.offset / COLS
    }

    /// Set the row
    pub fn set_row(&mut self, row: u8) {
        debug_assert!(row < ROWS);
        self.offset = self.col().saturating_add(row.saturating_mul(COLS));
    }

    /// Get the column
    pub const fn col(&self) -> u8 {
        self.offset % COLS
    }

    /// Set the column
    pub fn set_col(&mut self, col: u8) {
        debug_assert!(col < COLS);
        self.offset = self.row().saturating_mul(COLS).saturating_add(col);
    }

    /// Shift the offset by a number of rows and columns
    pub fn shift(&mut self, rows: i8, cols: i8) {
        self.offset += (rows as i16 * COLS as i16 + cols as i16) as u8;
    }

    /// Shift the offset by a number of rows
    pub fn row_shift(&mut self, rows: i8) {
        self.offset += (rows as i16 * COLS as i16) as u8;
    }

    /// Shift the offset by a number of columns
    pub fn col_shift(&mut self, cols: i8) {
        self.offset = (self.offset as i16 + cols as i16) as u8;
    }

    /// Is the offset less than the upper limit
    pub fn is_valid(&self) -> bool {
        self.offset < Self::UPPER_LIMIT
    }
}

impl<const R1: u8, const C1: u8, const R2: u8, const C2: u8> PartialEq<MatrixAddr<R2, C2>> for MatrixAddr<R1, C1> {
    fn eq(&self, rhs: &MatrixAddr<R2, C2>) -> bool {
        self.offset == rhs.offset
    }
}

impl<const R1: u8, const C1: u8, const R2: u8, const C2: u8> PartialOrd<MatrixAddr<R2, C2>> for MatrixAddr<R1, C1> {
    fn partial_cmp(&self, rhs: &MatrixAddr<R2, C2>) -> Option<core::cmp::Ordering> {
        if self.row() == rhs.row() && self.col() == rhs.col() {
            Some(core::cmp::Ordering::Equal)
        } else if self.row() > rhs.row() || (self.row() == rhs.row() && self.col() > rhs.col()) {
            Some(core::cmp::Ordering::Greater)
        } else {
            Some(core::cmp::Ordering::Less)
        }
    }
}

pub struct MatrixAddrIter<const ROWS: u8, const COLS: u8> {
    index: MatrixAddr<ROWS, COLS>,
}

impl<const ROWS: u8, const COLS: u8> Iterator for MatrixAddrIter<ROWS, COLS> {
    type Item = MatrixAddr<ROWS, COLS>;

    fn next(&mut self) -> Option<Self::Item> {
        let max = core::cmp::min(core::u8::MAX as usize, (ROWS as usize) * (COLS as usize));

        let index = self.index;

        if usize::from(index) < max {
            self.index += 1;
            Some(index)
        } else {
            None
        }
    }
}

impl<const ROWS: u8, const COLS: u8> IntoIterator for MatrixAddr<ROWS, COLS> {
    type Item = Self;
    type IntoIter = MatrixAddrIter<ROWS, COLS>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            index: Self::new(0),
        }
    }
}

impl<const ROWS: u8, const COLS: u8> From<&MatrixAddr<ROWS, COLS>> for u8 {
    fn from(addr: &MatrixAddr<ROWS, COLS>) -> Self {
        addr.row() * COLS + addr.col()
    }
}

impl<const ROWS: u8, const COLS: u8> From<MatrixAddr<ROWS, COLS>> for u8 {
    fn from(addr: MatrixAddr<ROWS, COLS>) -> Self {
        Self::from(&addr)
    }
}

impl<const ROWS: u8, const COLS: u8> From<&MatrixAddr<ROWS, COLS>> for usize {
    fn from(addr: &MatrixAddr<ROWS, COLS>) -> Self {
        u8::from(addr) as usize
    }
}

impl<const ROWS: u8, const COLS: u8> From<MatrixAddr<ROWS, COLS>> for usize {
    fn from(addr: MatrixAddr<ROWS, COLS>) -> Self {
        u8::from(&addr) as usize
    }
}

impl<const ROWS: u8, const COLS: u8> From<u8> for MatrixAddr<ROWS, COLS> {
    fn from(b: u8) -> Self {
        Self::new(b)
    }
}

impl<const ROWS: u8, const COLS: u8> AddAssign for MatrixAddr<ROWS, COLS> {
    fn add_assign(&mut self, rhs: Self) {
        *self = (u8::from(&*self) + u8::from(rhs)).into();
    }
}

impl<const ROWS: u8, const COLS: u8> AddAssign<u8> for MatrixAddr<ROWS, COLS> {
    fn add_assign(&mut self, rhs: u8) {
        *self = (u8::from(&*self) + rhs).into();
    }
}
