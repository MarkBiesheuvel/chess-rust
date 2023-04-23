use std::error::Error;
use chess::board::Board;

fn main() -> Result<(), Box<dyn Error>> {
    let board = Board::starting_position();

    println!("{}", board);

    Ok(())
}
