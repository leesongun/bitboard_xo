custom_error! { pub XOError
    TokenIndexOutOfRangeError{index: u32} = "Token index out of range ({index}), expected in range [0, 8]",
    AlreadyPlayedError{index: u32} = "Position index {index} has already been play",
    GameEndedError = "Move after ended",
}
pub type XOResult<T = ()> = Result<T, XOError>;

// Re-export
mod token;
pub use token::{XOToken, XOTokenWinState};

mod board;
pub use board::XOBoard;

pub mod xo_pos;
use xo_pos::XOPos;
//

use crate::board::BoardIter;
use crate::xo_pos::Index;
use custom_error::custom_error;
use std::fmt::{self, Display, Formatter};
use XOError::*;
use XOToken::*;

#[derive(Clone, Debug)]
pub struct XO {
    board: XOBoard,
    turn: XOToken,
    winner: Option<XOTokenWinState>,
}

impl XO {
    pub fn new() -> Self {
        XO {
            board: XOBoard::empty(),
            turn: X,
            winner: None,
        }
    }

    pub fn make_move(&mut self, xo_pos: impl XOPos) -> XOResult<Option<XOTokenWinState>> {
        if self.winner.is_some() {
            return Err(GameEndedError);
        }

        self.board = self.board.try_place_token(self.turn, xo_pos.to_index())?;
        self.swap_turn();

        self.winner = self.board.evaluate_winner();
        Ok(self.winner)
    }

    #[deprecated(since = "0.1.1", note = "use make_move instate")]
    pub fn push_move(&mut self, index: u32) -> XOResult<Option<XOTokenWinState>> {
        self.make_move(Index(index))
    }

    pub fn swap_turn(&mut self) {
        self.turn = self.turn.opposite_token();
    }

    pub fn reset(&mut self) {
        self.board = XOBoard::empty();
        self.turn = X;
        self.winner = None;
    }

    pub fn iter(&self) -> BoardIter {
        self.board.iter()
    }

    pub fn board(&self) -> XOBoard {
        self.board
    }

    pub fn board_mut(&mut self) -> &mut XOBoard {
        &mut self.board
    }

    pub fn turn(&self) -> XOToken {
        self.turn
    }
}

impl Default for XO {
    fn default() -> Self {
        XO::new()
    }
}

impl Display for XO {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{}'s turn", self.turn)?;
        if let Some(winner) = self.board.evaluate_winner() {
            writeln!(f, "{}'s winner", winner)?;
        } else {
            writeln!(f, "No winner")?;
        }
        self.board.fmt(f)?;
        Ok(())
    }
}
