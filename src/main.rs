use chess::Board;

mod chess;

fn main() {
    // Create a standard chess board
    let starting_position = Board::starting_position();
    dbg!(&starting_position);
    println!("{}", &starting_position);

    // Create a board with an ongoing game
    let ongoing_game = Board::fen("rnbqkb1r/ppp1pppp/8/3pP3/6n1/8/PPPPKPPP/RNBQ1BNR w kq d6 0 4");
    dbg!(&ongoing_game);
    println!("{}", &ongoing_game);
}
