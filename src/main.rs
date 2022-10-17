use chess::board::Board;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a standard chess board
    let starting_position = Board::starting_position();
    dbg!(&starting_position);
    println!("{}", &starting_position);

    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "rnbqkb1r/ppp1pppp/8/3pP3/6n1/8/PPPPKPPP/RNBQ1BNR w kq d6 0 4";
    let ongoing_game = Board::forsyth_edwards_notation(record)?;

    dbg!(&ongoing_game);
    println!("{}", &ongoing_game);

    Ok(())
}
