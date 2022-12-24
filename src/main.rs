use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "rnbqkbnr/ppp1p1pp/3p1p2/4P2Q/8/8/PPPP1PPP/RNB1KBNR b KQkq - 1 3";
    let mut board = Board::forsyth_edwards_notation(record)?;

    for _ in 0..10 {
        println!("{}", board);
        
        dbg!(board.is_in_check(board.active_color()));

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
