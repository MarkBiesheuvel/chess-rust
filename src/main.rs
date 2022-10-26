use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board
    let board = Board::starting_position();

    println!("{}", board);

    // Try out the new Board::white_pieces function
    for legal_move in board.legal_moves() {
        println!("{}", legal_move);
    }

    Ok(())
}
