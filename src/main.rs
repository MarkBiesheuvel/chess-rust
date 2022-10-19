use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "rn1k1bnr/2p1pp2/1p4p1/pBq1Rb2/2Q4p/2N2N2/PPPP1PPP/R1B3K1 w - a6 0 11";
    let board = Board::forsyth_edwards_notation(record)?;

    println!("{}", board);

    // Try out the new Board::white_pieces function
    for legal_move in board.legal_moves() {
        println!("{}", legal_move);
    }

    Ok(())
}
