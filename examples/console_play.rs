use bitboard_xo::*;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), XOError> {
    let mut game = XO::new();

    loop {
        // read index from console
        let index = loop {
            print!("Input index for {}: ", game.turn());
            stdout().flush().unwrap();

            let mut user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();

            match user_input.trim().parse() {
                Ok(a) => break a,
                Err(b) => println!("{}", b),
            }
        };

        // move it and match returned game state
        match game.play(XOPos::index(index)?) {
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
