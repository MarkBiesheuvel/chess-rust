use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "rnbqkbnr/pp2p1pp/8/1p1pPp2/8/5N2/PPPP1PPP/RNBQK2R w KQkq f6 0 5";
    let board = Board::forsyth_edwards_notation(record)?;

    println!("{}", board);

    // Try out the new Board::white_pieces function
    for legal_move in board.legal_moves() {
        println!("{}", legal_move);
    }

    Ok(())
}
