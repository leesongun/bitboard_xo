use std::fmt::{self, Display, Formatter};

/// Represent X,O of xo game
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum XOToken {
    X,
    O,
}

impl XOToken {
    /// Return an opposite xo token (X -> O, O -> X)
    ///
    /// This method does not mutate
    pub fn opposite_token(self) -> XOToken {
        match self {
            XOToken::X => XOToken::O,
            XOToken::O => XOToken::X,
        }
    }
}

impl Display for XOToken {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            XOToken::X => write!(f, "X"),
            XOToken::O => write!(f, "O"),
        }
    }
}

/// Represent type of ending of xo game, either `X` (win), `O` (win), or `State`(mate)
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum XOTokenWinState {
    /// Represent when game ended with X as winner
    X,
    /// Represent when game ended with O as winner
    O,
    /// Represent when game ended as stalemate
    Stale,
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

impl From<XOToken> for XOTokenWinState {
    fn from(token: XOToken) -> Self {
        match token {
            XOToken::X => XOTokenWinState::X,
            XOToken::O => XOTokenWinState::O,
        }
    }
}
