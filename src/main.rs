use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "2bqkbnr/1P3ppp/n7/3Pp3/8/2NQ1N2/PBP2PPP/R3K2R w KQk e6 0 12";
    let board = Board::forsyth_edwards_notation(record)?;

    println!("{}", board);

    // Try out the new Board::white_pieces function
    for legal_move in board.legal_moves() {
        println!("{}", legal_move);
    }

    Ok(())
}
