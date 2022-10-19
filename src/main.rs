use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    // let record = "3r4/1p3pk1/p4b2/P3p2p/1PP1P1b1/4K1p1/1R6/2r5 b - - 0 37";
    // let board = Board::forsyth_edwards_notation(record)?;
    let board = Board::starting_position();

    println!("{}", board);

    // Try out the new Board::white_pieces function
    for legal_move in board.legal_moves() {
        println!("{}", legal_move);
    }

    Ok(())
}
