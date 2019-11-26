use simple_xo::*;
use simple_xo::XOToken::*;
use std::io::stdin;
use text_io::*;

fn main() -> Result<(), XOError> {
    let mut game = XO::new();

    loop {
        let index = loop {
            match try_read!() {
                    Ok(a) => break a,
                    Err(b) => println!("{}", b),
            }
        };

        assert!(game.board_ref().check_sanity());

        match game.push_move(index) {
            Ok(Some(winner)) => {
                println!("{}", game);
                println!("winner = {:?}", winner);
                break
            },
            Ok(None) => println!("game continue"),
            Err(xo_err) =>  println!("Error: {}", xo_err),
        }

        println!("{}", game);
    }

    Ok(())
}
