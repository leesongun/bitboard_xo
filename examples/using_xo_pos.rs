use bitboard_xo::xo_pos::{ColRow, Index, RowCol};
use bitboard_xo::*;

fn main() -> XOResult {
    let board = XOBoard::empty();

    for i in 0..9 {
        println!(
            "At index {} \n{}",
            i,
            board.try_place_token(XOToken::X, Index(i))?
        );
    }

    for i in 0..3 {
        for j in 0..3 {
            println!(
                "At Row {}, Col {} \n{}",
                i,
                j,
                board.try_place_token(XOToken::O, RowCol(i, j))?
            );
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            println!(
                "At Col {}, Row {} \n{}",
                i,
                j,
                board.try_place_token(XOToken::X, ColRow(i, j))?
            );
        }
    }

    Ok(())
}
