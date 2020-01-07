#![doc(html_root_url = "https://docs.rs/bitboard_xo/1.1.0")]
//! A XO (aka tic-tac-toe/noughts and crosses) library
//! with focus on minimum memory usage without compromising performance.
//!
//! Minimum memory usage is achieve using [bitboard],
//!  a common data structure in chess, adapted to fit XO game.
//!
//! The current sizeof [`XO`] type is 32 bit, smaller than a pointer in most machine.
//!
//! # Examples
//! ```rust,no_run
//! use bitboard_xo::*;
//!
//! use std::error::Error;
//! use std::io::{stdin, stdout, Write};
//!
//! /// try read a u8 from console
//! fn read_u8() -> Result<u32, Box<dyn Error>> {
//!     let mut user_input = String::new();
//!     stdin().read_line(&mut user_input)?;
//!     Ok(user_input.trim().parse()?)
//! }
//!
//! /// prompt user for input to get xo position
//! fn get_xo_pos() -> XOPos {
//!     fn try_get_xo_pos() -> Result<XOPos, Box<dyn Error>> {
//!         // prompt user and read row from console
//!         print!("Input row : ");
//!         stdout().flush()?;
//!         let row = read_u8()?;
//!
//!         // prompt user and read col from console
//!         print!("Input col : ");
//!         stdout().flush()?;
//!         let col = read_u8()?;
//!
//!         // parse row, col into XOPos
//!         Ok(XOPos::row_col(row, col)?)
//!     };
//!
//!     loop {
//!         match try_get_xo_pos() {
//!             Ok(xo_pos) => break xo_pos,
//!             Err(err) => println!("{}", err),
//!         }
//!     }
//! }
//!
//! fn main() -> Result<(), XOError> {
//!     // create new XO game
//!     let mut game = XO::new();
//!
//!     loop {
//!         println!("{}", game);
//!         // get position from user's input
//!         let input_xo_pos = get_xo_pos();
//!
//!         // play at that position and match the returned game state
//!         match game.play(input_xo_pos) {
//!             // game end
//!             Ok(Some(game_result)) => {
//!                 println!("{}", game);
//!                 println!("game result => {:?}", game_result);
//!                 break;
//!             }
//!             // game continue
//!             Ok(None) => println!("game continue..."),
//!             // some game error occurred
//!             Err(xo_err) => println!("Error: {}", xo_err),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//! [`XO`]: struct.XO.html
//! [bitboard]: https://en.wikipedia.org/wiki/Bitboard

custom_error! {
/// Super Error combining all error in this crate ([`XOPosError`] and [`XOGameError`])
///
/// both [`XOPosError`] and [`XOGameError`] can convert to [`XOError`] with ? operator
///
/// ```rust
/// use bitboard_xo::XOError;
/// fn main() -> Result<(), XOError> {
///     use bitboard_xo::{XOPos, XO, XOPosError, XOGameError};
///
///     // ? auto convert XOPosError to XOError
///     let result_pos: Result<_, XOPosError> =
///         XOPos::col_row(1, 1);
///     let pos: XOPos = result_pos?;
///
///     // ? auto convert XOGameError to XOError
///     let mut game = XO::new();
///     let result_win_state: Result<_, XOGameError> =
///         game.play(pos);
///     let win_state = result_win_state?;
///
///     Ok(())
/// }
/// ```
///
/// [`XOPosError`]: xo_pos/enum.XOPosError.html
/// [`XOGameError`]: enum.XOGameError.html
/// [`XOError`]: enum.XOError.html
pub XOError
    PositionError{source: xo_pos::XOPosError} = "Invalid XO Position",
    GameError{source: board::XOGameError} = "Error occurred when trying to play (make a move)"
}
pub type XOResult<T = ()> = Result<T, XOError>;

// Re-export
mod token;
pub use token::{XOToken, XOTokenWinState};

mod board;
pub use board::{BoardIter, XOBoard, XOGameError};

pub mod xo_pos;
pub use xo_pos::{XOPos, XOPosError};
//

use crate::board::XOGameResult;
use custom_error::custom_error;
use std::fmt::{self, Display, Formatter};

/// Representing xo game. can be see as mutating version of [`XOBoard`].
///
/// It's implemented as wrapper of [`XOBoard`]
///
/// [`XOBoard`]: board/struct.XOBoard.html
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct XO {
    board: XOBoard,
}

impl XO {
    /// Construct a new game with empty board and X as starting player.
    ///
    /// Internally crate board with [`XOBoard::empty`] and wrap it.
    ///
    /// ```rust
    /// use bitboard_xo::{XO, XOBoard, XOToken};
    /// let game = XO::new();
    /// assert_eq!(game.board(), XOBoard::empty());
    /// assert_eq!(game.turn(), XOToken::X);
    /// ```
    ///
    /// [`XOBoard::empty`]: board/struct.XOBoard.html#method.empty
    pub fn new() -> Self {
        XO {
            board: XOBoard::empty(),
        }
    }

    /// Construct a game from [`XOBoard`].
    ///
    /// this function only wrap [`XOBoard`] without modifying anything in it
    ///
    /// [`XOBoard`]: board/struct.XOBoard.html
    pub fn from_board(board: XOBoard) -> Self {
        XO { board }
    }

    /// Play the game! This turn's player will try to play at position `pos`
    ///
    /// Assuming no error occured:
    ///
    /// * If after playing at `pos`, the game ended ->
    /// it return `Option::Some` containing the type of game ending as [`XOTokenWinState`]
    ///
    /// * If after playing at `pos`, the game doesn't end -> it return `Option::None`
    ///
    /// (the same as [`XO::win_state`])
    ///
    /// # Error
    /// May return Err variant [`XOGameError::AlreadyPlayedError`]
    /// if position `pos` isn't empty
    /// (most likely because this function get called with the same `pos` before)
    ///
    /// May return Err variant [`XOGameError::GameEndedError`]
    /// if game has already ended (X win, O win, or stalemate) before calling this function
    ///
    /// # Example
    /// ```rust
    /// # use bitboard_xo::XOResult;
    /// # fn main() -> XOResult {
    /// use bitboard_xo::{XO, XOPos, XOGameError, XOBoard, XOTokenWinState};
    /// use bitboard_xo::XOToken::*;
    ///
    /// /* create game with this configuration:
    /// X X .
    /// . O .
    /// . . .
    /// */
    /// let mut game = XO::from_board(XOBoard::from_maybe_token_array([
    ///     Some(X), Some(X), None,
    ///     None   , Some(O), None,
    ///     None   , None   , None,
    /// ]).swap_turn());
    ///
    /// // try playing at occupied position
    /// assert_eq!(
    ///     game.play(XOPos::row_col(0, 0)?),
    ///     Err(XOGameError::AlreadyPlayedError {index: 0})
    /// );
    ///
    /// // o play and game continue
    /// assert_eq!(game.turn(), O);
    /// assert_eq!(
    ///     game.play(XOPos::row_col(1, 0)?),
    ///     Ok(None)
    /// );
    ///
    /// // x play and win
    /// assert_eq!(game.turn(), X);
    /// assert_eq!(
    ///     game.play(XOPos::row_col(0, 2)?),
    ///     Ok(Some(XOTokenWinState::X))
    /// );
    ///
    /// // try to play after ended
    /// assert_eq!(
    ///     game.play(XOPos::row_col(2, 2)?),
    ///     Err(XOGameError::GameEndedError)
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`XOGameError::AlreadyPlayedError`]: enum.XOGameError.html#variant.AlreadyPlayedError
    /// [`XOGameError::GameEndedError`]: enum.XOGameError.html#variant.GameEndedError
    /// [`XO::win_state`]: struct.XO.html#method.win_state
    /// [`XOTokenWinState`]: enum.XOTokenWinState.html
    pub fn play(&mut self, pos: XOPos) -> XOGameResult<Option<XOTokenWinState>> {
        self.board = self.board.play(pos)?;
        Ok(self.board.win_state())
    }

    /// Swap current turn's play (player who going to play) (X -> O, O -> X)
    ///
    /// ```rust
    /// use bitboard_xo::{XO, XOToken};
    ///
    /// let mut game = XO::new();
    /// assert_eq!(game.turn(), XOToken::X);
    ///
    /// game.swap_turn();
    /// assert_eq!(game.turn(), XOToken::O);
    /// ```
    pub fn swap_turn(&mut self) {
        self.board = self.board.swap_turn();
    }

    /// get current turn's play (player who going to play)
    ///
    /// ```rust
    /// # use bitboard_xo::XOResult;
    /// # fn main() -> XOResult {
    /// use bitboard_xo::{XO, XOToken, XOPos};
    ///
    /// let mut game = XO::new();
    /// assert_eq!(game.turn(), XOToken::X);
    ///
    /// game.play(XOPos::row_col(1, 1)?);
    /// assert_eq!(game.turn(), XOToken::O);
    /// # Ok(())
    /// # }
    /// ```
    pub fn turn(self) -> XOToken {
        self.board.turn()
    }

    /// Get information about who (if any) won the game
    ///

    ///
    /// # Return
    /// * None if game didn't end yet
    /// * Some([XOTokenWinState::X])) if X won
    /// * Some([XOTokenWinState::O]) if O won
    /// * Some([XOTokenWinState::Stale]) if game ended in stalemate
    ///
    /// # Notes
    /// * This function doesn't directly calculate the winner.
    /// the calculation are already done when calling [`XO::play`] or [`XOBoard::play`],
    /// this function only retrieve the calculated information
    /// so it can be called repeatably without much performance penalty.
    ///
    /// * This function output are the same as in `Ok` variant of [`XO::play`]'s output.
    /// If this function output `None`
    /// then [`XO::play`] will output error variant of [`XOGameError::AlreadyPlayedError`]
    ///
    /// [`XO::play`]: struct.XO.html#method.play
    /// [`XOBoard::play`]: struct.XOBoard.html#method.play
    /// [`XOGameError::AlreadyPlayedError`]: enum.XOGameError.html#variant.AlreadyPlayedError
    /// [XOTokenWinState::X]: enum.XOTokenWinState.html#variant.X
    /// [XOTokenWinState::O]: enum.XOTokenWinState.html#variant.O
    /// [XOTokenWinState::Stale]: enum.XOTokenWinState.html#variant.Stale
    ///
    pub fn win_state(self) -> Option<XOTokenWinState> {
        self.board.win_state()
    }

    /// Reset the board to empty default board
    ///
    /// ```rust
    /// # use bitboard_xo::XOResult;
    /// # fn main() -> XOResult {
    /// use bitboard_xo::{XO, XOBoard, XOPos};
    ///
    /// let mut game = XO::new();
    ///
    /// game.play(XOPos::row_col(1, 1)?);
    /// assert_ne!(game, XO::new());
    /// assert_ne!(game.board(), XOBoard::empty());
    ///
    /// game.reset();
    /// assert_eq!(game, XO::new());
    /// assert_eq!(game.board(), XOBoard::empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn reset(&mut self) {
        self.board = XOBoard::empty();
    }

    /// Return immutable iterator through the board
    pub fn iter(self) -> BoardIter {
        self.board.iter()
    }

    /// Retrieve the wrapped [`XOBoard`].
    ///
    /// This method copy [`XOBoard`] out (since [`XOBoard`] implement Copy)
    /// so modifying the returned board will not affect the wrapped board.
    /// If you want to modify the board, see [`XO::board_mut`]
    ///
    ///
    /// [`XOBoard`]: struct.XOBoard.html
    /// [`XO::board_mut`]: struct.XO.html#method.board_mut
    pub fn board(self) -> XOBoard {
        self.board
    }

    /// Retrieve a mutable reference to wrapped [`XOBoard`].
    ///
    /// This method allowed modification of wrapped [`XO::board`]
    /// but might be less efficient
    /// than [`XO::board`] which use copying (usually, [`XO::board`]'s < pointer's size)
    /// but unable modify the wrapped board
    ///
    /// [`XOBoard`]: struct.XOBoard.html
    /// [`XO::board`]: struct.XO.html#method.board
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
