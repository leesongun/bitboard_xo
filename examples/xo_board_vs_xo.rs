use bitboard_xo::*;

fn main() -> XOResult {
    let mut board = XOBoard::empty();

    println!("0.\n {}", board);

    // XOBoard method create new board rather than mutate self
    // because XOBoard use so few byte, copy is more efficient

    // generate a board after playing at (0, 0) and display it
    let new_board = board.play(XOPos::row_col(0, 0)?)?;
    println!("1.\n{}", new_board);

    // to update board
    board = board.play(XOPos::row_col(0, 1)?)?;
    println!("2.\n{}", board);

    // because it return a board, chaining method is possible
    board = board
        .play(XOPos::row_col(0, 2)?)?
        .play(XOPos::row_col(1, 0)?)?
        .play(XOPos::row_col(0, 0)?)?;
    println!("3.\n{}", board);

    // To make returning board consistence,
    // unlike XO,using play will not return win state
    // use XOBoard.win_state to get win state
    println!("4.\n{}win state = {:?}", board, board.win_state());

    println!();

    // XO behave similar to XOBoard but mutate instate of create new

    // create new XO game
    let mut game = XO::new();
    println!("5.\n{}", game);

    // playing it
    game.play(XOPos::row_col(1, 0)?)?;
    println!("6.\n{}", game);

    // XO's play method do return win state
    game.play(XOPos::row_col(0, 0)?)?;
    game.play(XOPos::row_col(1, 1)?)?;
    game.play(XOPos::row_col(0, 1)?)?;
    let win_state = game.play(XOPos::row_col(1, 2)?)?;
    println!("7.\n{}win state = {:?}", game, win_state);

    // XO is implemented as wrapper of XOBoard
    // use XO.board to get inner XOBoard's copy or XO.board_mut to get mutable reference
    let token = game.board().token_at(XOPos::row_col(0, 1)?);
    println!("8.\n{}at (0,1) = {:?}", game, token);

    *game.board_mut() = XOBoard::empty();
    println!("9.\n{}", game);

    Ok(())
}
