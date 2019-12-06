custom_error!{ pub XOError
    TokenIndexOutOfRangeError{index: u32} = "Token index out of range ({index}), expected in range [0, 8]",
    AlreadyPlayedError{index: u32} = "Position index {index} has already been play",
    GameEndedError = "Move after ended",
}
pub type XOResult<T=()> = Result<T, XOError>;

// Re-export
mod token;
pub use token::{XOToken, XOTokenWinState};

mod board;
pub use board::XOBoard;
//

use XOToken::*;
use XOError::*;
use std::fmt::{self, Display, Formatter};
use custom_error::custom_error;

#[derive(Clone, Debug)]
pub struct XO {
    board: XOBoard,
    turn: XOToken,
    winner: Option<XOTokenWinState>,
}

impl XO {
    pub fn new() -> Self {
        XO { board: XOBoard::empty(), turn: X, winner: None }
    }

    pub fn push_move(&mut self, index: u32) -> XOResult<Option<XOTokenWinState>> {

        if self.winner.is_some() { Err(GameEndedError)?; }

        self.board.try_place_token(self.turn, index)?;
        self.swap_turn();

        self.winner = self.board.evaluate_winner();
        Ok(self.winner)
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