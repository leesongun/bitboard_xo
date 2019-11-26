#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum XOToken {
    X, O
}

use XOToken::*;

impl XOToken {
    pub fn opposite_token(self) -> XOToken {
        match self {
            X => O,
            O => X,
        }
    }
}

impl Display for XOToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            X => write!(f, "X"),
            O => write!(f, "O")
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum XOTokenWinState {
    X, O, Stale
}

impl Display for XOTokenWinState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            XOTokenWinState::X => write!(f, "X")?,
            XOTokenWinState::O => write!(f, "O")?,
            XOTokenWinState::Stale => write!(f, "Stale")?,
        }
        Ok(())
    }
}



pub struct XO {
    board: XOBoard,
    turn: XOToken,
    winner: Option<XOTokenWinState>,
}

impl XO {
    pub fn new() -> Self {
        XO { board: XOBoard::new(), turn: X, winner: None }
    }

    pub fn push_move(&mut self, index: u32) -> Result<Option<XOTokenWinState>, XOError> {

        if self.winner.is_some() { Err(GameEndedError)?; }
        if self.check_free_position(index)? { Err(AlreadyPlayedError {index})?; }

        self.board.replace_token(self.turn, index)?;
        self.winner = self.board.evaluate_winner();
        self.swap_turn();
        Ok(self.winner)
    }

    pub fn check_free_position(&self, index: u32) -> Result<bool, XOError>  {
        Ok(self.board.token_exist(X, index)? || self.board.token_exist(O, index)?)
    }

    pub fn swap_turn(&mut self) {
        self.turn = self.turn.opposite_token();
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.turn = X;
        self.winner = None;
    }

    pub fn board_ref(&self) -> &XOBoard { &self.board }

    pub fn board_mut(&mut self) -> &mut XOBoard { &mut self.board }

    pub fn turn(&self) -> XOToken {
        self.turn
    }
}

impl Display for XO {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{}'s turn", self.turn)?;
        if let Some(winner) = self.board.evaluate_winner() {
            writeln!(f, "{}'s winner", winner)?;
        }
        else {
            writeln!(f, "No winner")?;
        }
        self.board.fmt(f)?;
        Ok(())
    }
}

pub struct XOBoard {
    bit_board: u32,
}

const BIT_MASK: u32 = 0b111111111;
const BIT_SHIFT: u32 = 9;

use custom_error::custom_error;
custom_error!{ pub XOError
    TokenIndexOutOfRangeError{index: u32} = "Token index out of range ({index}), expected in range [0, 8]",
    AlreadyPlayedError{index: u32} = "Position index {index} has already been play",
    GameEndedError = "Move after ended",
}
use XOError::*;
use std::fmt::{Display, Formatter, Write};

impl XOBoard {
    pub fn new() -> Self {
        XOBoard {bit_board: 0}
    }

    pub fn reset(&mut self) {
        self.bit_board = 0;
    }

    pub fn x_bit(&self) -> u32 {
        self.bit_board & BIT_MASK
    }

    pub fn o_bit(&self) -> u32 {
        (self.bit_board >> BIT_SHIFT) & BIT_MASK
    }

    pub fn x_mask(&self, token_index: u32) -> Result<u32, XOError> {
        check_token_index(token_index)?;
        Ok(0b1 << token_index)
    }

    pub fn o_mask(&self, token_index: u32) -> Result<u32, XOError> {
        check_token_index(token_index)?;
        Ok(0b1 << (token_index + BIT_SHIFT))
    }

    pub fn token_mask(&self, token: XOToken, index: u32) -> Result<u32, XOError> {
        match token {
            X => self.x_mask(index),
            O => self.o_mask(index),
        }
    }

    pub fn token_bit(&self, token: XOToken) -> u32 {
        match token {
            X => self.x_bit(),
            O => self.o_bit(),
        }
    }

    pub fn token_exist(&self, token: XOToken, index: u32) -> Result<bool, XOError> {
        let token_bit = self.token_bit(token);
        check_token_index(index)?;
        Ok((token_bit & (0b1 << index))>>index == 1)

    }

    pub fn replace_token(&mut self, token: XOToken, token_index: u32) -> Result<(), XOError> {
        match token {
            XOToken::X => {
                self.bit_board = self.bit_board | self.x_mask(token_index)?;
            },
            XOToken::O => {
                self.bit_board = self.bit_board | self.o_mask(token_index)?;
            },
        }
        Ok(())
    }

    pub fn check_sanity(&self) -> bool {
        (self.o_bit() & self.x_bit()) == 0
    }

    pub fn evaluate_winner(&self) -> Option<XOTokenWinState> {
        let win_pattern_match = |bit| {
            const WIN_MASKES: [u32; 8] = [
                0b000000111 , 0b000111000, 0b111000000,
                0b100100100 , 0b010010010,  0b001001001,
                0b100010001 , 0b001010100,
            ];

            WIN_MASKES
                .iter()
                .find(|&mask| ((bit | !mask) & BIT_MASK) == 0b111111111)
                .map_or(Outcome::Failure, |_| Outcome::Success)

        };

        win_pattern_match(self.x_bit()).or_none(XOTokenWinState::X)
            .or_else(|| win_pattern_match(self.o_bit()).or_none(XOTokenWinState::O))
            .or_else(|| Outcome::from(self.bit_board==0b111111111111111111).or_none(XOTokenWinState::Stale))
    }
}

pub fn check_token_index(token_index: u32) -> Result<(), XOError> {
    if token_index >= 9 { Err(TokenIndexOutOfRangeError {index: token_index})? }
    Ok(())
}

use std::fmt;
use outcome::Outcome;

impl Display for XOBoard {

    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let x_bit = self.x_bit();
        let o_bit = self.o_bit();

        for j in 0..3 {
            for i in 0..3 {
                let index = i + 3*j;
                let mask = 0b1 << index;
                if (x_bit & mask)>>index == 1 { f.write_char('X')? }
                else if (o_bit & mask)>>index == 1 { f.write_char('O')? }
                else { f.write_char('.')? }
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}