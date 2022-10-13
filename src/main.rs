use chess::Board;

mod chess;

fn main() {
    // Create a standard chess board
    let board = Board::fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", board);

    // Create a board with an ongoing game
    let ongoing_game = Board::fen("r5k1/6p1/4bp1p/PP2N3/4p3/4P1P1/2r2P1P/R1B2RK1 b - - 0 24");
    println!("{}", ongoing_game);
}
