use custom_error::custom_error;
use std::convert::TryFrom;

custom_error! { pub XOPosError
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

#[derive(Copy, Clone, Debug)]
pub struct XOPos {
    index: u32,
}

impl XOPos {
    pub fn index(index: u32) -> Result<Self, XOPosError> {
        if index >= 9 {
            return Err(XOPosError::InvalidXOIndex { index });
        }

        Ok(XOPos { index })
    }
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
    pub fn col_row(col: u32, row: u32) -> Result<Self, XOPosError> {
        XOPos::row_col(row, col)
    }

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
