use chess::board::Board;

fn main() {
    // Create a standard chess board
    let starting_position = Board::starting_position();
    dbg!(&starting_position);
    println!("{}", &starting_position);

    // Create a board with an ongoing game
    // Since FEN break the spell checker, turn it of for the next line - cspell:disable-next
    let record = "rnbqkb1r/ppp1pppp/8/3pP3/6n1/8/PPPPKPPP/RNBQ1BNR w kq d6 0 4";
    let ongoing_game = match Board::forsyth_edwards_notation(record) {
        Ok(board) => board,
        Err(error) => panic!("{:?}", error),
    };

    dbg!(&ongoing_game);
    println!("{}", &ongoing_game);
}
