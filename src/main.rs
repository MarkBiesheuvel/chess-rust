use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "2bqkbnr/1P3ppp/n7/3Pp3/8/2NQ1N2/PBP2PPP/R3K2R w KQk e6 0 12";
    let mut board = Board::forsyth_edwards_notation(record)?;

    for _ in 0..10 {
        println!("{}", board);

        // Get all legal moves
        let legal_moves = board.legal_moves();

        // Just pick the first legal move
        // TODO: import Random crate
        match legal_moves.into_iter().nth(0) {
            Some(legal_move) => {
                println!("{}\n", legal_move);
                board.make_move(legal_move)?;
            }
            None => {
                println!("No legal moves left");
                break;
            }
        }
    }

    Ok(())
}
