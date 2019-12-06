use crate::token::{XOToken, XOTokenWinState};
use crate::{XOError::*, XOResult};

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

    pub fn reset(&mut self) {
        self.bit_board = 0;
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

    pub fn token_exist(self, token: XOToken, index: u32) -> XOResult<bool> {
        let token_bit = self.token_bit(token);
        check_token_index(index)?;
        Ok((token_bit & (0b1 << index)) >> index == 1)
    }

    pub fn set_token_at_index(&mut self, token: XOToken, index: u32) -> XOResult {
        match token {
            X => {
                self.bit_board |= self.x_mask(index)?;
            }
            O => {
                self.bit_board |= self.o_mask(index)?;
            }
        }
        Ok(())
    }

    pub fn try_place_token(&mut self, token: XOToken, index: u32) -> XOResult {
        if !self.check_free_position(index)? {
            return Err(AlreadyPlayedError { index });
        }
        self.set_token_at_index(token, index)?;
        Ok(())
    }

    pub fn check_free_position(self, index: u32) -> XOResult<bool> {
        Ok(!self.token_exist(X, index)? && !self.token_exist(O, index)?)
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
                let mask = 0b1 << index;

                if (x_bit & mask) >> index == 1 {
                    f.write_char('X')?
                } else if (o_bit & mask) >> index == 1 {
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
