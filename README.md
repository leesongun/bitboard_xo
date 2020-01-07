[![Build Status](https://travis-ci.org/thanadolps/bitboard_xo.svg?branch=master)](https://travis-ci.org/thanadolps/bitboard_xo)

# bitboard_xo

A XO (aka tic-tac-toe/noughts and crosses) library
with focus on minimum memory usage without compromising performance.

Minimum memory usage is achieve using [bitboard],
 a common data structure in chess, adapted to fit XO game.

The current sizeof XO type is 32 bit, smaller than a pointer in most machine.

## Examples
```rust
use bitboard_xo::*;

use std::error::Error;
use std::io::{stdin, stdout, Write};

/// try read a u8 from console
fn read_u8() -> Result<u32, Box<dyn Error>> {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().parse()?)
}

/// prompt user for input to get xo position
fn get_xo_pos() -> XOPos {
    fn try_get_xo_pos() -> Result<XOPos, Box<dyn Error>> {
        // prompt user and read row from console
        print!("Input row : ");
        stdout().flush()?;
        let row = read_u8()?;

        // prompt user and read col from console
        print!("Input col : ");
        stdout().flush()?;
        let col = read_u8()?;

        // parse row, col into XOPos
        Ok(XOPos::row_col(row, col)?)
    };

    loop {
        match try_get_xo_pos() {
            Ok(xo_pos) => break xo_pos,
            Err(err) => println!("{}", err),
        }
    }
}

fn main() -> Result<(), XOError> {
    // create new XO game
    let mut game = XO::new();

    loop {
        println!("{}", game);
        // get position from user's input
        let input_xo_pos = get_xo_pos();

        // play at that position and match the returned game state
        match game.play(input_xo_pos) {
            // game end
            Ok(Some(game_result)) => {
                println!("{}", game);
                println!("game result => {:?}", game_result);
                break;
            }
            // game continue
            Ok(None) => println!("game continue..."),
            // some game error occurred
            Err(xo_err) => println!("Error: {}", xo_err),
        }
    }

    Ok(())
}
```
[bitboard]: https://en.wikipedia.org/wiki/Bitboard

License: MIT
