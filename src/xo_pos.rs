use custom_error::custom_error;
use std::convert::TryFrom;

custom_error! {
/// Error use when attempting to constructing invalid [`XOPos`]
///
/// [`XOPos`]: struct.XOPos.html
pub XOPosError
    InvalidXOIndex {index: u32} =
        "invalid xo's index: {index}, expecting integer in [0, 8] (correspond to xo's cell,
        \n 0\t1\t2
        \n 3\t4\t5
        \n 6\t7\t8
        \n )",
    InvalidXORow {row: u32} =
        "invalid xo's row: {row}, expecting one of 0, 1, 2 (correspond to top, middle, and bottom row)",
    InvalidXOCol {col: u32} =
        "invalid xo's column: {col}, expecting one of 0, 1, 2 (correspond to left, middle, and right column)",
}

/// Position in xo-grid
#[derive(Copy, Clone, Debug)]
pub struct XOPos {
    index: u32,
}

impl XOPos {
    /// Create [`XOPos`] from index,
    /// starting 0 at top left corner and counting up in book reading direction (see grid below)
    ///
    /// Index grid:
    ///
    /// ```text
    /// | 0 | 1 | 2 |
    /// | 3 | 4 | 5 |
    /// | 6 | 7 | 8 |
    /// ```
    ///
    /// # Error
    /// If invalid index is passed in (index >= 9), the error variant [`XOPosError::InvalidXOIndex`] will be returned
    ///
    /// [`XOPos`]: struct.XOPos.html
    /// [`XOPosError::InvalidXOIndex`]: ../xo_pos/enum.XOPosError.html#variant.InvalidXOIndex
    ///
    pub fn index(index: u32) -> Result<Self, XOPosError> {
        if index >= 9 {
            return Err(XOPosError::InvalidXOIndex { index });
        }

        Ok(XOPos { index })
    }

    /// Create [`XOPos`] from (row, column) position.
    ///
    /// (row, column) grid:
    ///
    /// ```text
    /// | 0,0 | 0,1 | 0,2 |
    /// | 1,0 | 1,1 | 1,2 |
    /// | 2,0 | 2,1 | 2,2 |
    /// ```
    ///
    /// # Error
    /// If invalid row (row >= 3) or column (column >= 3) is passed in,
    /// the error variant [`XOPosError::InvalidXORow`] or [`XOPosError::InvalidXOCol`]
    /// will be returned correspondingly
    ///
    /// [`XOPos`]: struct.XOPos.html
    /// [`XOPosError::InvalidXORow`]: ../xo_pos/enum.XOPosError.html#variant.InvalidXORow
    /// [`XOPosError::InvalidXOCol`]: ../xo_pos/enum.XOPosError.html#variant.InvalidXOCol
    ///
    pub fn row_col(row: u32, col: u32) -> Result<Self, XOPosError> {
        if row >= 3 {
            return Err(XOPosError::InvalidXORow { row });
        }
        if col >= 3 {
            return Err(XOPosError::InvalidXOCol { col });
        }

        Ok(XOPos {
            index: 3 * row + col,
        })
    }

    /// Create [`XOPos`] from (column, row) position
    ///
    /// (column, row) grid:
    ///
    /// ```text
    /// | 0,0 | 1,0 | 2,0 |
    /// | 0,1 | 1,1 | 2,1 |
    /// | 0,2 | 1,2 | 2,2 |
    /// ```
    ///
    /// # Error
    /// If invalid row (row >= 3) or column (column >= 3) is passed in,
    /// the error variant [`XOPosError::InvalidXORow`] or [`XOPosError::InvalidXOCol`]
    /// will be returned correspondingly
    ///
    /// [`XOPos`]: struct.XOPos.html
    /// [`XOPosError::InvalidXORow`]: ../xo_pos/enum.XOPosError.html#variant.InvalidXORow
    /// [`XOPosError::InvalidXOCol`]: ../xo_pos/enum.XOPosError.html#variant.InvalidXOCol
    ///
    pub fn col_row(col: u32, row: u32) -> Result<Self, XOPosError> {
        XOPos::row_col(row, col)
    }

    /// Retrieve index inside this [`XOPos`],
    /// guaranteed to be one of 0,1,2,3,4,5,6,7,8
    ///
    /// Index grid:
    ///
    /// ```text
    /// | 0 | 1 | 2 |
    /// | 3 | 4 | 5 |
    /// | 6 | 7 | 8 |
    /// ```
    ///
    /// [`XOPos`]: struct.XOPos.html
    pub fn as_index(self) -> u32 {
        debug_assert!(self.index < 9);
        self.index
    }
}

impl TryFrom<u32> for XOPos {
    type Error = XOPosError;

    fn try_from(value: u32) -> Result<Self, XOPosError> {
        XOPos::index(value)
    }
}
