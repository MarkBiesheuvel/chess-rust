use chess::Board;
use chess::Piece;
use chess::PieceColor::{Black, White};
use chess::PieceKind::{Bishop, King, Knight, Pawn, Queen, Rook};

mod chess;

fn main() {
    let mut pieces = [
        Piece::new(White, Bishop),
        Piece::new(Black, King),
        Piece::new(White, Knight),
        Piece::new(Black, Pawn),
        Piece::new(White, Queen),
        Piece::new(Black, Rook),
    ];

    // Move all pieces
    for piece in &mut pieces {
        piece.update();
    }

    // Show legal moves of all pieces
    for piece in &pieces {
        println!("{:?}", piece.legal_moves());
    }

    let board = Board::new();

    println!("{}", board);
}
