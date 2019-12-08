use bitboard_xo::*;

fn main() -> XOResult {
    /*
        X X O
        . O X
        . . .
    */
    let board = XOBoard::from_maybe_token_array([
        Some(XOToken::X),
        Some(XOToken::X),
        Some(XOToken::O),
        None,
        Some(XOToken::O),
        Some(XOToken::X),
        None,
        None,
        None,
    ]);

    println!("{}", board);

    // using iterator, order like reading direction
    println!("Using Iterator: ");
    for maybe_token in board.iter() {
        match maybe_token {
            Some(XOToken::X) => print!("X "),
            Some(XOToken::O) => print!("O "),
            None => print!(". "),
        }
    }
    println!("\n");

    // using indexing function, this might be a bit slower than using iterator
    println!("Using token_at: ");
    for i in 0..9 {
        match board.token_at(XOPos::index(i)?) {
            Some(XOToken::X) => print!("X "),
            Some(XOToken::O) => print!("O "),
            None => print!(". "),
        }
    }

    Ok(())
}
