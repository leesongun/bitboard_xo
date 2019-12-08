custom_error! { pub XOError
    PositionError{source: xo_pos::XOPosError} = "Invalid XO Position",
    GameError{source: board::XOGameError} = "Error occurred when trying to play (make a move)"
}
pub type XOResult<T = ()> = Result<T, XOError>;

// Re-export
mod token;
pub use token::{XOToken, XOTokenWinState};

mod board;
pub use board::{XOBoard, XOGameError};

pub mod xo_pos;
pub use xo_pos::{XOPos, XOPosError};
//

use crate::board::{BoardIter, XOGameResult};
use custom_error::custom_error;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct XO {
    board: XOBoard,
}

impl XO {
    pub fn new() -> Self {
        XO {
            board: XOBoard::empty(),
        }
    }

    pub fn from_board(board: XOBoard) -> Self {
        XO { board }
    }

    pub fn play(&mut self, pos: XOPos) -> XOGameResult<Option<XOTokenWinState>> {
        self.board = self.board.play(pos)?;
        Ok(self.board.win_state())
    }

    pub fn swap_turn(&mut self) {
        self.board = self.board.swap_turn();
    }

    pub fn turn(self) -> XOToken {
        self.board.turn()
    }

    pub fn win_state(self) -> Option<XOTokenWinState> {
        self.board.win_state()
    }

    pub fn reset(&mut self) {
        self.board = XOBoard::empty();
    }

    pub fn iter(self) -> BoardIter {
        self.board.iter()
    }

    pub fn board(self) -> XOBoard {
        self.board
    }

    pub fn board_mut(&mut self) -> &mut XOBoard {
        &mut self.board
    }
}

impl Default for XO {
    fn default() -> Self {
        XO::new()
    }
}

impl Display for XO {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        self.board.fmt(f)
    }
}

impl From<XOBoard> for XO {
    fn from(board: XOBoard) -> Self {
        XO::from_board(board)
    }
}
