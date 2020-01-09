use crate::token::{XOToken, XOTokenWinState};
use crate::xo_pos::XOPos;

use custom_error::custom_error;
use outcome::Outcome;
use std::fmt::{self, Display, Formatter, Write};
use XOToken::*;

custom_error! {
/// Error use when trying to do invalid play
///
/// Currently only use in [`XO::play`] and [`XOBoard::play`]
///
/// [`XO::play`]: struct.XOBoard.html/#method.play
/// [`XOBoard::play`]: struct.XOBoard.html/#method.play
#[derive(Eq, PartialEq)]
pub XOGameError
    /// Error of trying to play at position that isn't empty (position that already been play)
    AlreadyPlayedError{index: u32} = "Position index {index} has already been play",
    /// Error of trying to play after game ended
    GameEndedError = "attempt to play after game's ended",
}
use XOGameError::*;
pub type XOGameResult<T = ()> = Result<T, XOGameError>;

#[must_use = "XOBoard's method doesn't mutate but return a new board"]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct XOBoard {
    bit_board: u32,
}

const BIT_MASK: u32 = 0b111_111_111;
const BIT_SHIFT: u32 = 9;

impl XOBoard {
    pub fn new(bits: u32) -> Self {
        XOBoard { bit_board: bits }
    }

    pub fn empty() -> Self {
        XOBoard::new(0)
    }

    pub fn from_maybe_token_array(arr: [Option<XOToken>; 9]) -> Self {
        let mut bits = 0_u32;
        for (index, maybe_token) in arr.iter().enumerate() {
            match maybe_token {
                Some(XOToken::X) => bits |= 0b1_u32 << index as u32,
                Some(XOToken::O) => bits |= 0b1_u32 << (index as u32 + BIT_SHIFT),
                None => {}
            }
        }
        XOBoard::new(bits)
    }

    pub fn x_bit(self) -> u32 {
        self.bit_board & BIT_MASK
    }

    pub fn o_bit(self) -> u32 {
        (self.bit_board >> BIT_SHIFT) & BIT_MASK
    }

    fn x_mask(self, token_pos: XOPos) -> u32 {
        0b1 << token_pos.as_index()
    }

    fn o_mask(self, token_pos: XOPos) -> u32 {
        0b1 << (token_pos.as_index() + BIT_SHIFT)
    }

    pub fn turn(self) -> XOToken {
        if ((self.bit_board >> (2 * BIT_SHIFT)) & 0b1) == 1 {
            XOToken::O
        } else {
            XOToken::X
        }
    }

    pub fn swap_turn(self) -> XOBoard {
        XOBoard::new(self.bit_board ^ (0b1 << (2 * BIT_SHIFT)))
    }

    pub fn win_state(self) -> Option<XOTokenWinState> {
        match (self.bit_board >> (2 * BIT_SHIFT + 1)) & 0b111 {
            0b000 => None,
            0b011 => Some(XOTokenWinState::X),
            0b101 => Some(XOTokenWinState::O),
            0b111 => Some(XOTokenWinState::Stale),
            _ => unreachable!(),
        }
    }

    pub fn game_ended(self) -> bool {
        (self.bit_board >> (2 * BIT_SHIFT + 1)) & 0b1 == 1
    }

    pub fn set_win_state(self, win_state: Option<XOTokenWinState>) -> XOBoard {
        let win_state_bits =
            // TODO: make this on XOTokenWinState Type
            match win_state {
                None => 0b000,
                Some(XOTokenWinState::X) => 0b011,
                Some(XOTokenWinState::O) => 0b101,
                Some(XOTokenWinState::Stale) => 0b111,
            };

        XOBoard::new(self.bit_board | (win_state_bits << (2 * BIT_SHIFT + 1)))
    }

    fn token_mask(self, token: XOToken, pos: XOPos) -> u32 {
        match token {
            X => self.x_mask(pos),
            O => self.o_mask(pos),
        }
    }

    pub fn token_bit(self, token: XOToken) -> u32 {
        match token {
            X => self.x_bit(),
            O => self.o_bit(),
        }
    }

    pub fn token_exist(self, token: XOToken, pos: XOPos) -> bool {
        let index = pos.as_index();
        let token_bit = self.token_bit(token);
        (token_bit & (0b1 << index)) >> index == 1
    }

    pub fn set(self, token: XOToken, pos: XOPos) -> XOBoard {
        XOBoard::new(
            (self.bit_board | self.token_mask(token, pos))
                & (!self.token_mask(token.opposite_token(), pos)),
        )
    }

    pub fn play(self, pos: XOPos) -> XOGameResult<XOBoard> {
        if self.game_ended() {
            return Err(GameEndedError);
        }

        if !self.check_free_position(pos) {
            return Err(AlreadyPlayedError {
                index: pos.as_index(),
            });
        }

        let board = self.set(self.turn(), pos);
        let board = board.set_win_state(board.evaluate_winner()).swap_turn();
        Ok(board)
    }

    pub fn check_free_position(self, pos: XOPos) -> bool {
        !self.token_exist(X, pos) && !self.token_exist(O, pos)
    }

    pub fn token_at(self, pos: XOPos) -> Option<XOToken> {
        let index = pos.as_index();
        if (self.x_bit() >> index & 0b1) == 1 {
            Some(XOToken::X)
        } else if (self.o_bit() >> index & 0b1) == 1 {
            Some(XOToken::O)
        } else {
            None
        }
    }

    pub fn check_sanity(self) -> bool {
        (self.o_bit() & self.x_bit()) == 0
    }

    pub fn evaluate_winner(self) -> Option<XOTokenWinState> {
        let win_pattern_match = |bit| {
            const WIN_MASKES: [u32; 8] = [
                0b000_000_111,
                0b000_111_000,
                0b111_000_000,
                0b100_100_100,
                0b010_010_010,
                0b001_001_001,
                0b100_010_001,
                0b001_010_100,
            ];

            WIN_MASKES
                .iter()
                .find(|&mask| ((bit | !mask) & BIT_MASK) == 0b1_1111_1111)
                .map_or(Outcome::Failure, |_| Outcome::Success)
        };

        win_pattern_match(self.x_bit())
            .or_none(XOTokenWinState::X)
            .or_else(|| win_pattern_match(self.o_bit()).or_none(XOTokenWinState::O))
            .or_else(|| {
                Outcome::from(self.x_bit() | self.o_bit() == 0b111_111_111)
                    .or_none(XOTokenWinState::Stale)
            })
    }

    pub fn iter(self) -> BoardIter {
        BoardIter::new(self)
    }
}

impl Display for XOBoard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        if let Some(winner) = self.win_state() {
            match winner {
                XOTokenWinState::Stale => writeln!(f, "Game Ended In Stale Mate")?,
                XOTokenWinState::X | XOTokenWinState::O => {
                    writeln!(f, "Game Ended: {}'s winner", winner)?
                }
            }
        } else {
            writeln!(f, "{}'s Turn", self.turn())?;
        }

        let x_bit = self.x_bit();
        let o_bit = self.o_bit();

        for j in 0..3 {
            for i in 0..3 {
                let index = i + 3 * j;

                if (x_bit >> index & 0b1) == 1 {
                    f.write_char('X')?
                } else if (o_bit >> index & 0b1) == 1 {
                    f.write_char('O')?
                } else {
                    f.write_char('.')?
                }
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

/// Immutable iterator through [`XOBoard`]
///
/// # Construction
/// This struct can't be construct directly but can construct with [`XO::iter`] or [`XOBoard::iter`]
///
/// # Iteration
/// Will Iterate starting from top-left cell and continue in book reading direction.
///
/// (iteration order from 0 -> 8)
/// ```text
/// | 0 | 1 | 2 |
/// | 3 | 4 | 5 |
/// | 6 | 7 | 8 |
/// ```
///
/// Each iteration, yield
/// * None on empty cell
/// * Some([XOToken::X]) on cell with X
/// * Some([XOToken::O]) on cell with O
///
///
///
/// # Examples
/// ```rust
/// # use bitboard_xo::XOResult;
/// # fn main() -> XOResult {
/// use bitboard_xo::{XO, XOPos, XOGameError, XOBoard, XOTokenWinState};
/// use bitboard_xo::XOToken::*;
/// use bitboard_xo::BoardIter;
///
/// /* create game with this board configuration:
/// X X O
/// . . .
/// . . .
/// */
/// let board = XOBoard::from_maybe_token_array([
///     Some(X), Some(X), Some(O),
///     None   , None   , None   ,
///     None   , None   , None   ,
/// ]);
///
/// // note that XO also has the same method signature
/// let iter = board.iter();
/// assert!(iter.eq(
///     [Some(X), Some(X), Some(O), None, None, None, None, None, None].iter().copied()
/// ));
///
/// # Ok(())
/// # }
/// ```
///
/// # Notes
/// * Since [`XO`] and [`XOBoard`] are copy type,
/// this struct doesn't store mutable reference but store a copy [`XOBoard`].
/// so modifying the original [`XOBoard`] while iterating with this struct is both possible and safe
/// (but might be unexpected)
///
/// [`XOBoard`]: struct.XOBoard.html
/// [`XO`]: struct.XO.html
/// [`XO::iter`]: struct.XO.html#method.iter
/// [`XOBoard::iter`]: struct.XOBoard.html#method.iter
/// [XOToken::X]: enum.XOToken.html#variant.X
/// [XOToken::O]: enum.XOToken.html#variant.O
pub struct BoardIter {
    board: XOBoard,
    current_index: u32,
}

impl BoardIter {
    fn new(board: XOBoard) -> BoardIter {
        BoardIter {
            board,
            current_index: 0,
        }
    }
}

impl Iterator for BoardIter {
    type Item = Option<XOToken>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= 9 {
            return None;
        }

        let maybe_token = if (self.board.x_bit() >> self.current_index & 0b1) == 1 {
            Some(XOToken::X)
        } else if (self.board.o_bit() >> self.current_index & 0b1) == 1 {
            Some(XOToken::O)
        } else {
            None
        };

        self.current_index += 1;
        Some(maybe_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XOResult;

    #[test]
    fn x_win() -> XOResult {
        let mut board = XOBoard::empty();
        board = board.play(XOPos::index(0)?)?;
        board = board.play(XOPos::index(1)?)?;
        board = board.play(XOPos::index(2)?)?;
        board = board.play(XOPos::index(3)?)?;
        board = board.play(XOPos::index(4)?)?;
        board = board.play(XOPos::index(5)?)?;
        board = board.play(XOPos::index(6)?)?;

        assert!(board.game_ended());
        assert_eq!(board.win_state(), Some(XOTokenWinState::X));
        Ok(())
    }

    #[test]
    fn stale() -> XOResult {
        let mut board = XOBoard::empty();
        board = board.play(XOPos::row_col(0, 0)?)?;
        board = board.play(XOPos::row_col(1, 1)?)?;
        board = board.play(XOPos::row_col(2, 2)?)?;
        board = board.play(XOPos::row_col(0, 2)?)?;
        board = board.play(XOPos::row_col(0, 1)?)?;
        board = board.play(XOPos::row_col(1, 0)?)?;
        board = board.play(XOPos::row_col(2, 0)?)?;
        board = board.play(XOPos::row_col(2, 1)?)?;
        board = board.play(XOPos::row_col(1, 2)?)?;

        assert!(board.game_ended());
        assert_eq!(board.win_state(), Some(XOTokenWinState::Stale));
        Ok(())
    }
}
