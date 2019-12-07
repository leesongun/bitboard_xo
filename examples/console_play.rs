use bitboard_xo::*;
use std::io::{stdout, Write};
use text_io::*;

fn main() -> Result<(), XOError> {
    let mut game = XO::new();

    loop {
        // read index from console
        let index = loop {
            print!("Input index for {}: ", game.turn());
            stdout().flush().unwrap();
            let user_input = try_read!();
            match user_input {
                Ok(a) => break a,
                Err(b) => println!("{}", b),
            }
        };

        // move it and match returned game state
        match game.make_move(xo_pos::Index(index)) {
            Ok(Some(winner)) => {
                println!("{}", game);
                println!("winner = {:?}", winner);
                break;
            }
            Ok(None) => println!("game continue..."),
            Err(xo_err) => println!("Error: {}", xo_err),
        }

        println!("{}", game);
    }

    Ok(())
}
