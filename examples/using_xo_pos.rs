use bitboard_xo::*;

fn main() -> XOResult {
    let board = XOBoard::empty();

    for i in 0..9 {
        println!("At index {} \n{}", i, board.play(XOPos::index(i)?)?);
    }

    for i in 0..3 {
        for j in 0..3 {
            println!(
                "At Row {}, Col {} \n{}",
                i,
                j,
                board.swap_turn().play(XOPos::row_col(i, j)?)?
            );
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            println!(
                "At Col {}, Row {} \n{}",
                i,
                j,
                board.play(XOPos::col_row(i, j)?)?
            );
        }
    }

    Ok(())
}
