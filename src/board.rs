use crate::token::{XOToken, XOTokenWinState};
use crate::{XOError::*, XOResult};

use crate::xo_pos::XOPos;
use outcome::Outcome;
use std::fmt::{self, Display, Formatter, Write};
use XOToken::*;

#[derive(Copy, Clone, Debug)]
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

    pub fn x_mask(self, token_index: u32) -> XOResult<u32> {
        check_token_index(token_index)?;
        Ok(0b1 << token_index)
    }

    pub fn o_mask(self, token_index: u32) -> XOResult<u32> {
        check_token_index(token_index)?;
        Ok(0b1 << (token_index + BIT_SHIFT))
    }

    pub fn token_mask(self, token: XOToken, index: u32) -> XOResult<u32> {
        match token {
            X => self.x_mask(index),
            O => self.o_mask(index),
        }
    }

    pub fn token_bit(self, token: XOToken) -> u32 {
        match token {
            X => self.x_bit(),
            O => self.o_bit(),
        }
    }

    fn token_exist_at_index(self, token: XOToken, index: u32) -> XOResult<bool> {
        let token_bit = self.token_bit(token);
        check_token_index(index)?;
        Ok((token_bit & (0b1 << index)) >> index == 1)
    }

    pub fn token_exist(self, token: XOToken, pos: impl XOPos) -> XOResult<bool> {
        self.token_exist_at_index(token, pos.to_index())
    }

    fn set_token_at_index(self, token: XOToken, index: u32) -> XOResult<XOBoard> {
        Ok(XOBoard::new(
            self.bit_board | self.token_mask(token, index)?,
        ))
    }

    fn try_place_token_at_index(self, token: XOToken, index: u32) -> XOResult<XOBoard> {
        if !self.check_free_position(index)? {
            return Err(AlreadyPlayedError { index });
        }
        Ok(self.set_token_at_index(token, index)?)
    }

    pub fn set_token(self, token: XOToken, pos: impl XOPos) -> XOResult<XOBoard> {
        self.set_token_at_index(token, pos.to_index())
    }

    pub fn try_place_token(self, token: XOToken, pos: impl XOPos) -> XOResult<XOBoard> {
        self.try_place_token_at_index(token, pos.to_index())
    }

    fn check_free_position_at_index(self, index: u32) -> XOResult<bool> {
        Ok(!self.token_exist(X, index)? && !self.token_exist(O, index)?)
    }

    pub fn check_free_position(self, pos: impl XOPos) -> XOResult<bool> {
        self.check_free_position_at_index(pos.to_index())
    }

    fn token_at_index(self, index: u32) -> XOResult<Option<XOToken>> {
        check_token_index(index)?;
        Ok(if (self.x_bit() >> index & 0b1) == 1 {
            Some(XOToken::X)
        } else if (self.o_bit() >> index & 0b1) == 1 {
            Some(XOToken::O)
        } else {
            None
        })
    }

    pub fn token_at(self, pos: impl XOPos) -> XOResult<Option<XOToken>> {
        self.token_at_index(pos.to_index())
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
                Outcome::from(self.bit_board == 0b11_1111_1111_1111_1111)
                    .or_none(XOTokenWinState::Stale)
            })
    }

    pub fn iter(self) -> BoardIter {
        BoardIter::new(self)
    }
}

pub fn check_token_index(token_index: u32) -> XOResult {
    if token_index >= 9 {
        return Err(TokenIndexOutOfRangeError { index: token_index });
    }
    Ok(())
}

impl Display for XOBoard {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
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
